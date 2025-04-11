use std::error::Error;
use std::path::Path;
use std::sync::Arc;
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::Mutex;
use tracing::{info, error};

mod api;
mod container;
mod image;
mod network;
mod volume;
mod utils;

// デーモンの状態を管理する構造体
struct RockerDaemon {
    container_manager: container::Manager,
    image_manager: image::Manager,
    network_manager: network::Manager,
    volume_manager: volume::Manager,
}

impl RockerDaemon {
    fn new() -> Self {
        RockerDaemon {
            container_manager: container::Manager::new(),
            image_manager: image::Manager::new(),
            network_manager: network::Manager::new(),
            volume_manager: volume::Manager::new(),
        }
    }

    // 初期化処理
    async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        // 各マネージャの初期化
        self.container_manager.init().await?;
        self.image_manager.init().await?;
        self.network_manager.init().await?;
        self.volume_manager.init().await?;
        
        // デフォルトネットワークの作成
        if !self.network_manager.exists("bridge").await? {
            self.network_manager.create_default_bridge().await?;
        }
        
        Ok(())
    }
    
    // 既存コンテナの復元
    async fn restore_containers(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Restoring existing containers...");
        let containers = self.container_manager.list_all().await?;
        
        for container in containers {
            if container.auto_restart() {
                match self.container_manager.start(&container.id).await {
                    Ok(_) => info!("Restored container: {}", container.id),
                    Err(e) => error!("Failed to restore container {}: {}", container.id, e),
                }
            }
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // ロギングの初期化
    tracing_subscriber::fmt::init();
    
    info!("Starting rocker daemon...");
    
    // データディレクトリの作成
    let data_dir = Path::new("/var/lib/rocker");
    if !data_dir.exists() {
        std::fs::create_dir_all(data_dir)?;
    }
    
    // デーモンの初期化
    let daemon = Arc::new(Mutex::new(RockerDaemon::new()));
    {
        let mut daemon_guard = daemon.lock().await;
        daemon_guard.init().await?;
        daemon_guard.restore_containers().await?;
    }
    
    // Unixソケットの作成
    let socket_path = Path::new("/var/run/rocker.sock");
    if socket_path.exists() {
        std::fs::remove_file(socket_path)?;
    }
    
    let listener = UnixListener::bind(socket_path)?;
    info!("Listening on unix://{}", socket_path.display());
    
    // HTTP APIサーバーの起動
    let api_daemon = Arc::clone(&daemon);
    tokio::spawn(async move {
        api::start_http_server(api_daemon).await.unwrap();
    });
    
    // Unixソケット接続の処理
    while let Ok((stream, _)) = listener.accept().await {
        let daemon_clone = Arc::clone(&daemon);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, daemon_clone).await {
                error!("Error handling connection: {}", e);
            }
        });
    }
    
    Ok(())
}

async fn handle_connection(stream: UnixStream, daemon: Arc<Mutex<RockerDaemon>>) -> Result<(), Box<dyn Error>> {
    // ここでクライアントからのリクエストを処理
    // JSONベースのプロトコル実装
    
    Ok(())
} 
