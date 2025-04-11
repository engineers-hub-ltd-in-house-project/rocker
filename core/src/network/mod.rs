use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Network represents a container network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    /// Network ID (UUID)
    pub id: String,
    /// Network name
    pub name: String,
    /// Network driver
    pub driver: NetworkDriver,
    /// Network options
    pub options: HashMap<String, String>,
    /// Network configuration
    pub config: NetworkConfig,
    /// Connected containers
    pub containers: HashMap<String, NetworkContainer>,
    /// Creation time
    pub created_at: DateTime<Utc>,
}

impl Network {
    /// Create a new network
    pub fn new(name: String, driver: NetworkDriver, config: NetworkConfig) -> Self {
        Network {
            id: Uuid::new_v4().to_string(),
            name,
            driver,
            options: HashMap::new(),
            config,
            containers: HashMap::new(),
            created_at: Utc::now(),
        }
    }
}

/// Network driver
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkDriver {
    /// Bridge network
    Bridge,
    /// Host network
    Host,
    /// Null network
    None,
    /// Overlay network
    Overlay,
    /// Macvlan network
    Macvlan,
}

impl std::fmt::Display for NetworkDriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let driver_str = match self {
            NetworkDriver::Bridge => "bridge",
            NetworkDriver::Host => "host",
            NetworkDriver::None => "none",
            NetworkDriver::Overlay => "overlay",
            NetworkDriver::Macvlan => "macvlan",
        };
        write!(f, "{}", driver_str)
    }
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Subnet of the network
    pub subnet: String,
    /// Gateway of the network
    pub gateway: String,
    /// IP range of the network
    pub ip_range: Option<String>,
    /// Enable IPv6
    pub enable_ipv6: bool,
    /// Internal network (not exposed to outside)
    pub internal: bool,
    /// Enable IP masquerade
    pub enable_ip_masquerade: bool,
    /// Labels
    pub labels: HashMap<String, String>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            subnet: "172.17.0.0/16".to_string(),
            gateway: "172.17.0.1".to_string(),
            ip_range: None,
            enable_ipv6: false,
            internal: false,
            enable_ip_masquerade: true,
            labels: HashMap::new(),
        }
    }
}

/// NetworkContainer represents a container connected to a network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkContainer {
    /// Container ID
    pub container_id: String,
    /// IP address assigned to the container
    pub ip_address: String,
    /// MAC address assigned to the container
    pub mac_address: String,
    /// Network aliases for the container
    pub aliases: Vec<String>,
}

/// IPAM (IP Address Management) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamConfig {
    /// IPAM driver
    pub driver: String,
    /// IPAM options
    pub options: HashMap<String, String>,
    /// IPAM config
    pub config: Vec<IpamPoolConfig>,
}

impl Default for IpamConfig {
    fn default() -> Self {
        IpamConfig {
            driver: "default".to_string(),
            options: HashMap::new(),
            config: Vec::new(),
        }
    }
}

/// IPAM pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamPoolConfig {
    /// Subnet
    pub subnet: String,
    /// IP range
    pub ip_range: Option<String>,
    /// Gateway
    pub gateway: Option<String>,
    /// Auxiliary addresses
    pub aux_addresses: HashMap<String, String>,
} 
