use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use tracing::{info, error, warn};

// Compose設定ファイルの構造体
#[derive(Debug, Serialize, Deserialize)]
pub struct ComposeConfig {
    version: String,
    services: HashMap<String, ServiceConfig>,
    #[serde(default)]
    networks: HashMap<String, NetworkConfig>,
    #[serde(default)]
    volumes: HashMap<String, VolumeConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    image: Option<String>,
    build: Option<BuildConfig>,
    command: Option<Command>,
    #[serde(default)]
    environment: Environment,
    #[serde(default)]
    volumes: Vec<String>,
    #[serde(default)]
    ports: Vec<String>,
    #[serde(default)]
    depends_on: Vec<String>,
    #[serde(rename = "restart", default)]
    restart_policy: String,
    #[serde(default)]
    networks: Vec<String>,
    #[serde(default)]
    labels: HashMap<String, String>,
    #[serde(default)]
    healthcheck: Option<HealthcheckConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Command {
    String(String),
    List(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Environment {
    List(Vec<String>),
    Map(HashMap<String, String>),
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Map(HashMap::new())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BuildConfig {
    String(String),
    Object {
        context: String,
        #[serde(rename = "dockerfile", default)]
        rockerfile: Option<String>,
        args: Option<HashMap<String, String>>,
    },
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NetworkConfig {
    driver: Option<String>,
    #[serde(default)]
    external: bool,
    #[serde(default)]
    ipam: Option<IpamConfig>,
    #[serde(default)]
    driver_opts: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamConfig {
    driver: Option<String>,
    config: Option<Vec<IpamPoolConfig>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamPoolConfig {
    subnet: Option<String>,
    gateway: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VolumeConfig {
    driver: Option<String>,
    #[serde(default)]
    external: bool,
    #[serde(default)]
    driver_opts: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthcheckConfig {
    test: Command,
    interval: Option<String>,
    timeout: Option<String>,
    retries: Option<u32>,
    start_period: Option<String>,
}

pub struct ComposeProject {
    config: ComposeConfig,
    project_name: String,
    project_dir: std::path::PathBuf,
}

impl ComposeProject {
    pub fn new<P: AsRef<Path>>(
        config_path: P,
        project_name: Option<String>,
    ) -> Result<Self, Box<dyn Error>> {
        let config_path = config_path.as_ref();
        let config_content = std::fs::read_to_string(config_path)?;
        
        let config: ComposeConfig = serde_yaml::from_str(&config_content)?;
        
        // プロジェクト名とディレクトリを取得
        let project_dir = config_path.parent().unwrap_or(Path::new(".")).to_path_buf();
        let project_name = project_name.unwrap_or_else(|| {
            project_dir
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("default")
                .to_string()
        });
        
        Ok(ComposeProject {
            config,
            project_name,
            project_dir,
        })
    }
    
    pub async fn up(&self, detached: bool) -> Result<(), Box<dyn Error>> {
        info!("Starting project: {}", self.project_name);
        
        // ネットワークの作成
        self.create_networks().await?;
        
        // ボリュームの作成
        self.create_volumes().await?;
        
        // 依存関係グラフの構築
        let service_order = self.resolve_dependencies()?;
        
        // サービスの起動
        for service_name in service_order {
            self.start_service(&service_name, detached).await?;
        }
        
        if !detached {
            info!("Services started. Press Ctrl+C to stop...");
            // 非デタッチモードの場合、Ctrl+Cを待ち受ける
            tokio::signal::ctrl_c().await?;
            self.down(false).await?;
        }
        
        Ok(())
    }
    
    pub async fn down(&self, remove_volumes: bool) -> Result<(), Box<dyn Error>> {
        info!("Stopping project: {}", self.project_name);
        
        // サービスの停止と削除（依存関係の逆順）
        let service_order = self.resolve_dependencies()?;
        for service_name in service_order.iter().rev() {
            self.stop_service(service_name).await?;
        }
        
        // ネットワークの削除
        self.remove_networks().await?;
        
        // ボリュームの削除（オプションで）
        if remove_volumes {
            self.remove_volumes().await?;
        }
        
        Ok(())
    }
    
    async fn create_networks(&self) -> Result<(), Box<dyn Error>> {
        info!("Creating networks for project {}", self.project_name);
        
        for (network_name, network_config) in &self.config.networks {
            // 外部ネットワークはスキップ
            if network_config.external {
                continue;
            }
            
            let full_name = format!("{}_{}",  self.project_name, network_name);
            info!("Creating network: {}", full_name);
            
            // NetworkManager APIを使ってネットワーク作成
            // ここでは簡易化のためプロセス実行を使用
            // 実際の実装ではrockerデーモンのAPIを使用する
            
            // TODO: デーモンAPIでネットワーク作成
        }
        
        Ok(())
    }
    
    async fn create_volumes(&self) -> Result<(), Box<dyn Error>> {
        info!("Creating volumes for project {}", self.project_name);
        
        for (volume_name, volume_config) in &self.config.volumes {
            // 外部ボリュームはスキップ
            if volume_config.external {
                continue;
            }
            
            let full_name = format!("{}_{}",  self.project_name, volume_name);
            info!("Creating volume: {}", full_name);
            
            // VolumeManager APIを使ってボリューム作成
            // TODO: デーモンAPIでボリューム作成
        }
        
        Ok(())
    }
    
    fn resolve_dependencies(&self) -> Result<Vec<String>, Box<dyn Error>> {
        // トポロジカルソートで依存関係を解決
        let mut result = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut temp_mark = std::collections::HashSet::new();
        
        // すべてのサービスを処理
        for service_name in self.config.services.keys() {
            if !visited.contains(service_name) {
                self.visit_node(service_name, &mut visited, &mut temp_mark, &mut result)?;
            }
        }
        
        Ok(result)
    }
    
    fn visit_node(
        &self,
        node: &str,
        visited: &mut std::collections::HashSet<String>,
        temp_mark: &mut std::collections::HashSet<String>,
        result: &mut Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        // 一時マークが付いている場合、循環依存がある
        if temp_mark.contains(node) {
            return Err(format!("Circular dependency detected: {}", node).into());
        }
        
        // 訪問済みノードはスキップ
        if visited.contains(node) {
            return Ok(());
        }
        
        // 一時マークを付ける
        temp_mark.insert(node.to_string());
        
        // 依存関係を処理
        if let Some(service) = self.config.services.get(node) {
            for dep in &service.depends_on {
                if !visited.contains(dep.as_str()) {
                    self.visit_node(dep, visited, temp_mark, result)?;
                }
            }
        }
        
        // 一時マークを外す
        temp_mark.remove(node);
        
        // 訪問済みマークを付ける
        visited.insert(node.to_string());
        result.push(node.to_string());
        
        Ok(())
    }
    
    async fn start_service(&self, service_name: &str, detached: bool) -> Result<(), Box<dyn Error>> {
        let service = self.config.services.get(service_name)
            .ok_or_else(|| format!("Service not found: {}", service_name))?;
            
        info!("Starting service: {}", service_name);
        
        // イメージをビルドまたはプル
        let image = if let Some(build_config) = &service.build {
            self.build_image(service_name, build_config).await?
        } else if let Some(image) = &service.image {
            image.clone()
        } else {
            return Err(format!("Service {} has neither image nor build specified", service_name).into());
        };
        
        // 環境変数の準備
        let env_vars = match &service.environment {
            Environment::List(list) => {
                let mut env_map = HashMap::new();
                for item in list {
                    if let Some((key, value)) = item.split_once('=') {
                        env_map.insert(key.to_string(), value.to_string());
                    }
                }
                env_map
            },
            Environment::Map(map) => map.clone(),
        };
        
        // コンテナ名を生成
        let container_name = format!("{}_{}", self.project_name, service_name);
        
        // コンテナを作成して起動
        // TODO: デーモンAPIでコンテナ作成と起動
        
        Ok(())
    }
    
    async fn build_image(&self, service_name: &str, build_config: &BuildConfig) -> Result<String, Box<dyn Error>> {
        info!("Building image for service: {}", service_name);
        
        let (context, rockerfile) = match build_config {
            BuildConfig::String(context) => (context.clone(), None),
            BuildConfig::Object { context, rockerfile, .. } => (context.clone(), rockerfile.clone()),
        };
        
        // コンテキストパスを解決
        let context_path = self.project_dir.join(context);
        
        // Rockerfileパスを解決
        let rockerfile_path = match rockerfile {
            Some(file) => context_path.join(file),
            None => context_path.join("Rockerfile"),
        };
        
        if !rockerfile_path.exists() {
            return Err(format!("Rockerfile not found at {}", rockerfile_path.display()).into());
        }
        
        // イメージタグを生成
        let image_tag = format!("{}_{}", self.project_name, service_name);
        
        // イメージをビルド
        // TODO: デーモンAPIでイメージビルド
        
        Ok(image_tag)
    }
    
    async fn stop_service(&self, service_name: &str) -> Result<(), Box<dyn Error>> {
        info!("Stopping service: {}", service_name);
        
        // コンテナ名を生成
        let container_name = format!("{}_{}", self.project_name, service_name);
        
        // コンテナを停止して削除
        // TODO: デーモンAPIでコンテナ停止と削除
        
        Ok(())
    }
    
    async fn remove_networks(&self) -> Result<(), Box<dyn Error>> {
        info!("Removing networks for project {}", self.project_name);
        
        for network_name in self.config.networks.keys() {
            let full_name = format!("{}_{}",  self.project_name, network_name);
            info!("Removing network: {}", full_name);
            
            // NetworkManager APIを使ってネットワーク削除
            // TODO: デーモンAPIでネットワーク削除
        }
        
        Ok(())
    }
    
    async fn remove_volumes(&self) -> Result<(), Box<dyn Error>> {
        info!("Removing volumes for project {}", self.project_name);
        
        for volume_name in self.config.volumes.keys() {
            let full_name = format!("{}_{}",  self.project_name, volume_name);
            info!("Removing volume: {}", full_name);
            
            // VolumeManager APIを使ってボリューム削除
            // TODO: デーモンAPIでボリューム削除
        }
        
        Ok(())
    }
}

// Composeツールのエントリーポイント
pub async fn up_command(
    file: Option<&str>,
    project_name: Option<&str>,
    detached: bool,
) -> Result<(), Box<dyn Error>> {
    let config_path = file.unwrap_or("rocker-compose.yaml");
    let project = ComposeProject::new(config_path, project_name.map(|s| s.to_string()))?;
    project.up(detached).await
}

pub async fn down_command(
    file: Option<&str>,
    project_name: Option<&str>,
    remove_volumes: bool,
) -> Result<(), Box<dyn Error>> {
    let config_path = file.unwrap_or("rocker-compose.yaml");
    let project = ComposeProject::new(config_path, project_name.map(|s| s.to_string()))?;
    project.down(remove_volumes).await
} 
