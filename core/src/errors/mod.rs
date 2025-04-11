use thiserror::Error;

/// RockerError represents all possible errors in the Rocker container engine
#[derive(Error, Debug)]
pub enum RockerError {
    /// Container errors
    #[error("Container error: {0}")]
    Container(#[from] ContainerError),

    /// Image errors
    #[error("Image error: {0}")]
    Image(#[from] ImageError),

    /// Network errors
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),

    /// Volume errors
    #[error("Volume error: {0}")]
    Volume(#[from] VolumeError),

    /// Daemon errors
    #[error("Daemon error: {0}")]
    Daemon(String),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization/Deserialization errors
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// Generic errors
    #[error("{0}")]
    Generic(String),
}

/// ContainerError represents container-related errors
#[derive(Error, Debug)]
pub enum ContainerError {
    /// Container not found
    #[error("Container not found: {0}")]
    NotFound(String),

    /// Container already exists
    #[error("Container already exists: {0}")]
    AlreadyExists(String),

    /// Failed to start container
    #[error("Failed to start container: {0}")]
    Start(String),

    /// Failed to stop container
    #[error("Failed to stop container: {0}")]
    Stop(String),

    /// Failed to remove container
    #[error("Failed to remove container: {0}")]
    Remove(String),

    /// Failed to create container
    #[error("Failed to create container: {0}")]
    Create(String),

    /// Container is already running
    #[error("Container is already running: {0}")]
    AlreadyRunning(String),

    /// Container is not running
    #[error("Container is not running: {0}")]
    NotRunning(String),

    /// Failed to execute command in container
    #[error("Failed to execute command in container: {0}")]
    Exec(String),

    /// Failed to get container logs
    #[error("Failed to get container logs: {0}")]
    Logs(String),

    /// Container runtime error
    #[error("Container runtime error: {0}")]
    Runtime(String),
}

/// ImageError represents image-related errors
#[derive(Error, Debug)]
pub enum ImageError {
    /// Image not found
    #[error("Image not found: {0}")]
    NotFound(String),

    /// Image already exists
    #[error("Image already exists: {0}")]
    AlreadyExists(String),

    /// Failed to pull image
    #[error("Failed to pull image: {0}")]
    Pull(String),

    /// Failed to push image
    #[error("Failed to push image: {0}")]
    Push(String),

    /// Failed to build image
    #[error("Failed to build image: {0}")]
    Build(String),

    /// Failed to remove image
    #[error("Failed to remove image: {0}")]
    Remove(String),

    /// Failed to tag image
    #[error("Failed to tag image: {0}")]
    Tag(String),

    /// Failed to save image
    #[error("Failed to save image: {0}")]
    Save(String),

    /// Failed to load image
    #[error("Failed to load image: {0}")]
    Load(String),

    /// Failed to parse image reference
    #[error("Failed to parse image reference: {0}")]
    Reference(String),

    /// Registry error
    #[error("Registry error: {0}")]
    Registry(String),
}

/// NetworkError represents network-related errors
#[derive(Error, Debug)]
pub enum NetworkError {
    /// Network not found
    #[error("Network not found: {0}")]
    NotFound(String),

    /// Network already exists
    #[error("Network already exists: {0}")]
    AlreadyExists(String),

    /// Failed to create network
    #[error("Failed to create network: {0}")]
    Create(String),

    /// Failed to remove network
    #[error("Failed to remove network: {0}")]
    Remove(String),

    /// Failed to connect container to network
    #[error("Failed to connect container to network: {0}")]
    Connect(String),

    /// Failed to disconnect container from network
    #[error("Failed to disconnect container from network: {0}")]
    Disconnect(String),

    /// IP allocation error
    #[error("IP allocation error: {0}")]
    IpAllocation(String),

    /// Invalid network configuration
    #[error("Invalid network configuration: {0}")]
    InvalidConfig(String),
}

/// VolumeError represents volume-related errors
#[derive(Error, Debug)]
pub enum VolumeError {
    /// Volume not found
    #[error("Volume not found: {0}")]
    NotFound(String),

    /// Volume already exists
    #[error("Volume already exists: {0}")]
    AlreadyExists(String),

    /// Failed to create volume
    #[error("Failed to create volume: {0}")]
    Create(String),

    /// Failed to remove volume
    #[error("Failed to remove volume: {0}")]
    Remove(String),

    /// Failed to mount volume
    #[error("Failed to mount volume: {0}")]
    Mount(String),

    /// Failed to unmount volume
    #[error("Failed to unmount volume: {0}")]
    Unmount(String),

    /// Volume is in use
    #[error("Volume is in use: {0}")]
    InUse(String),

    /// Invalid volume driver
    #[error("Invalid volume driver: {0}")]
    InvalidDriver(String),
} 
