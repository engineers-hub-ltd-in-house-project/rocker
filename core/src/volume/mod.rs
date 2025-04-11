use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

/// Volume represents a container volume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    /// Volume ID (UUID)
    pub id: String,
    /// Volume name
    pub name: String,
    /// Volume driver
    pub driver: VolumeDriver,
    /// Mount point of the volume
    pub mountpoint: PathBuf,
    /// Volume configuration
    pub config: VolumeConfig,
    /// Creation time
    pub created_at: DateTime<Utc>,
    /// Volume scope (local, global)
    pub scope: VolumeScope,
    /// Volume status
    pub status: HashMap<String, String>,
    /// Labels
    pub labels: HashMap<String, String>,
}

impl Volume {
    /// Create a new volume
    pub fn new(name: String, driver: VolumeDriver, config: VolumeConfig) -> Self {
        Volume {
            id: Uuid::new_v4().to_string(),
            name,
            driver,
            mountpoint: PathBuf::new(),
            config,
            created_at: Utc::now(),
            scope: VolumeScope::Local,
            status: HashMap::new(),
            labels: HashMap::new(),
        }
    }
}

/// Volume driver
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VolumeDriver {
    /// Local driver
    Local,
    /// Custom driver
    Custom(String),
}

impl std::fmt::Display for VolumeDriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VolumeDriver::Local => write!(f, "local"),
            VolumeDriver::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Volume scope
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VolumeScope {
    /// Local scope
    Local,
    /// Global scope
    Global,
}

impl std::fmt::Display for VolumeScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VolumeScope::Local => write!(f, "local"),
            VolumeScope::Global => write!(f, "global"),
        }
    }
}

/// Volume configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeConfig {
    /// Driver options
    pub driver_opts: HashMap<String, String>,
    /// Labels
    pub labels: HashMap<String, String>,
}

impl Default for VolumeConfig {
    fn default() -> Self {
        VolumeConfig {
            driver_opts: HashMap::new(),
            labels: HashMap::new(),
        }
    }
}

/// Mount options for a volume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountOptions {
    /// Type of the mount (bind, volume, tmpfs)
    pub mount_type: String,
    /// Source of the mount (host path or volume name)
    pub source: String,
    /// Target path in the container
    pub target: String,
    /// Read-only flag
    pub read_only: bool,
    /// Consistency level (default, consistent, cached, delegated)
    pub consistency: Option<String>,
    /// Bind propagation (rprivate, private, rshared, shared, rslave, slave)
    pub bind_propagation: Option<String>,
    /// If the source does not exist, should it be created?
    pub create_source: bool,
} 
