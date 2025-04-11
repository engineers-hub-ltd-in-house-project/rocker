use serde::{Deserialize, Serialize};

/// ContainerState represents the state of a container
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContainerState {
    /// Container is created but not started
    Created,
    /// Container is running
    Running,
    /// Container is paused
    Paused,
    /// Container is being restarted
    Restarting,
    /// Container has exited
    Exited,
    /// Container has been marked for removal
    Removing,
    /// Container is stopped
    Stopped,
    /// Container is dead (failed to stop or remove)
    Dead,
}

impl ContainerState {
    /// Returns true if the container is running
    pub fn is_running(&self) -> bool {
        matches!(self, ContainerState::Running)
    }

    /// Returns true if the container is paused
    pub fn is_paused(&self) -> bool {
        matches!(self, ContainerState::Paused)
    }

    /// Returns true if the container has exited
    pub fn is_exited(&self) -> bool {
        matches!(self, ContainerState::Exited)
    }

    /// Returns true if the container is stopped
    pub fn is_stopped(&self) -> bool {
        matches!(self, ContainerState::Stopped)
    }

    /// Returns true if the container is created but not started
    pub fn is_created(&self) -> bool {
        matches!(self, ContainerState::Created)
    }

    /// Returns true if the container is being restarted
    pub fn is_restarting(&self) -> bool {
        matches!(self, ContainerState::Restarting)
    }

    /// Returns true if the container is being removed
    pub fn is_removing(&self) -> bool {
        matches!(self, ContainerState::Removing)
    }

    /// Returns true if the container is dead
    pub fn is_dead(&self) -> bool {
        matches!(self, ContainerState::Dead)
    }
}

impl std::fmt::Display for ContainerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_str = match self {
            ContainerState::Created => "created",
            ContainerState::Running => "running",
            ContainerState::Paused => "paused",
            ContainerState::Restarting => "restarting",
            ContainerState::Exited => "exited",
            ContainerState::Removing => "removing",
            ContainerState::Stopped => "stopped",
            ContainerState::Dead => "dead",
        };
        write!(f, "{}", state_str)
    }
} 
