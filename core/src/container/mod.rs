use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

mod state;
pub use state::*;

/// Mount represents a mounted volume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mount {
    /// Type of the mount (bind, volume, tmpfs)
    pub mount_type: MountType,
    /// Source of the mount (host path or volume name)
    pub source: String,
    /// Destination path in the container
    pub destination: String,
    /// Read-only flag
    pub read_only: bool,
    /// Mount propagation mode
    pub propagation: Option<PropagationMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MountType {
    Bind,
    Volume,
    Tmpfs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropagationMode {
    Private,
    Shared,
    Slave,
}

/// NetworkEndpoint represents a container's connection to a network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEndpoint {
    /// Network ID
    pub network_id: String,
    /// IP address assigned to the container in this network
    pub ip_address: String,
    /// Network aliases for the container
    pub aliases: Vec<String>,
}

/// ContainerConfig holds the configuration of a container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerConfig {
    /// Name of the image to use
    pub image: String,
    /// Command to run in the container
    pub cmd: Option<Vec<String>>,
    /// Working directory inside the container
    pub working_dir: Option<String>,
    /// Environment variables as key-value pairs
    pub env: HashMap<String, String>,
    /// Exposed ports
    pub exposed_ports: Vec<u16>,
    /// Host to container port mappings
    pub port_bindings: HashMap<u16, u16>,
    /// Volume mounts
    pub mounts: Vec<Mount>,
    /// Restart policy
    pub restart_policy: RestartPolicy,
    /// Resource limits
    pub resource_limits: ResourceLimits,
    /// Network mode
    pub network_mode: NetworkMode,
    /// Privileged mode
    pub privileged: bool,
    /// Additional capabilities to add
    pub cap_add: Vec<String>,
    /// Capabilities to drop
    pub cap_drop: Vec<String>,
    /// User to run the container as (user:group)
    pub user: Option<String>,
    /// Hostname of the container
    pub hostname: Option<String>,
    /// Domain name of the container
    pub domainname: Option<String>,
    /// Container labels
    pub labels: HashMap<String, String>,
}

impl Default for ContainerConfig {
    fn default() -> Self {
        ContainerConfig {
            image: String::new(),
            cmd: None,
            working_dir: None,
            env: HashMap::new(),
            exposed_ports: Vec::new(),
            port_bindings: HashMap::new(),
            mounts: Vec::new(),
            restart_policy: RestartPolicy::No,
            resource_limits: ResourceLimits::default(),
            network_mode: NetworkMode::Bridge,
            privileged: false,
            cap_add: Vec::new(),
            cap_drop: Vec::new(),
            user: None,
            hostname: None,
            domainname: None,
            labels: HashMap::new(),
        }
    }
}

/// Resource limits for a container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// CPU limit in percentage (0-100)
    pub cpu_percent: Option<u8>,
    /// Memory limit in bytes
    pub memory_bytes: Option<u64>,
    /// Swap limit in bytes
    pub memory_swap_bytes: Option<u64>,
    /// IO read limit in bytes per second
    pub io_read_bps: Option<u64>,
    /// IO write limit in bytes per second
    pub io_write_bps: Option<u64>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        ResourceLimits {
            cpu_percent: None,
            memory_bytes: None,
            memory_swap_bytes: None,
            io_read_bps: None,
            io_write_bps: None,
        }
    }
}

/// Network mode for a container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMode {
    /// Default bridge network
    Bridge,
    /// Host networking
    Host,
    /// No networking
    None,
    /// Container networking (use another container's network)
    Container(String),
    /// Custom network
    Custom(String),
}

/// Restart policy for a container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestartPolicy {
    /// Do not restart automatically
    No,
    /// Always restart
    Always,
    /// Restart on failure
    OnFailure {
        /// Maximum retry count (None means unlimited)
        max_retry: Option<u32>,
    },
    /// Restart unless stopped
    UnlessStopped,
}

/// Container represents a running or stopped container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    /// Container ID (UUID)
    pub id: String,
    /// Container name
    pub name: String,
    /// Container configuration
    pub config: ContainerConfig,
    /// Current state of the container
    pub state: ContainerState,
    /// Time when the container was created
    pub created_at: DateTime<Utc>,
    /// Time when the container was started (if applicable)
    pub started_at: Option<DateTime<Utc>>,
    /// Time when the container was finished (if applicable)
    pub finished_at: Option<DateTime<Utc>>,
    /// Exit code of the container (if applicable)
    pub exit_code: Option<i32>,
    /// Process ID of the container (if running)
    pub pid: Option<i32>,
    /// IP address of the container
    pub ip_address: Option<String>,
    /// Networks that the container is connected to
    pub networks: HashMap<String, NetworkEndpoint>,
}

impl Container {
    /// Create a new container
    pub fn new(name: String, config: ContainerConfig) -> Self {
        Container {
            id: Uuid::new_v4().to_string(),
            name,
            config,
            state: ContainerState::Created,
            created_at: Utc::now(),
            started_at: None,
            finished_at: None,
            exit_code: None,
            pid: None,
            ip_address: None,
            networks: HashMap::new(),
        }
    }

    /// Check if the container should be automatically restarted
    pub fn auto_restart(&self) -> bool {
        match &self.config.restart_policy {
            RestartPolicy::Always => true,
            RestartPolicy::OnFailure { max_retry: _ } => {
                if let Some(exit_code) = self.exit_code {
                    if exit_code != 0 {
                        return true;
                    }
                }
                false
            }
            RestartPolicy::UnlessStopped => self.state != ContainerState::Stopped,
            RestartPolicy::No => false,
        }
    }
} 
