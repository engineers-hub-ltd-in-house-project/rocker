use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Image represents a container image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    /// Image ID (content-addressable hash)
    pub id: String,
    /// Repository name
    pub repo: Option<String>,
    /// Image tag
    pub tag: Option<String>,
    /// Time when the image was created
    pub created_at: DateTime<Utc>,
    /// Size of the image in bytes
    pub size: u64,
    /// Layers that make up the image
    pub layers: Vec<ImageLayer>,
    /// Configuration of the image
    pub config: ImageConfig,
    /// Parent image ID
    pub parent_id: Option<String>,
    /// Labels
    pub labels: HashMap<String, String>,
}

impl Image {
    /// Returns the full name of the image (repo:tag)
    pub fn full_name(&self) -> Option<String> {
        match (&self.repo, &self.tag) {
            (Some(repo), Some(tag)) => Some(format!("{}:{}", repo, tag)),
            (Some(repo), None) => Some(format!("{}:latest", repo)),
            _ => None,
        }
    }

    /// Returns the names of the image (repo:tag pairs)
    pub fn names(&self) -> Vec<String> {
        let mut names = Vec::new();
        if let Some(name) = self.full_name() {
            names.push(name);
        }
        names
    }

    /// Returns true if the image has the given repo and tag
    pub fn matches(&self, repo: &str, tag: Option<&str>) -> bool {
        if let Some(image_repo) = &self.repo {
            if image_repo != repo {
                return false;
            }
        } else {
            return false;
        }

        match (tag, &self.tag) {
            (Some(tag), Some(image_tag)) => tag == image_tag,
            (None, _) => true,
            (Some(_), None) => false,
        }
    }

    /// Returns true if the image matches the given name (repo:tag)
    pub fn matches_name(&self, name: &str) -> bool {
        if let Some(colon_pos) = name.find(':') {
            let repo = &name[0..colon_pos];
            let tag = &name[colon_pos + 1..];
            self.matches(repo, Some(tag))
        } else {
            self.matches(name, None)
        }
    }
}

/// ImageLayer represents a layer of an image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageLayer {
    /// Layer ID (content-addressable hash)
    pub id: String,
    /// Diff ID (content-addressable hash of the uncompressed layer)
    pub diff_id: String,
    /// Size of the layer in bytes
    pub size: u64,
    /// Path to the layer on disk
    pub path: PathBuf,
    /// Creation time of the layer
    pub created_at: DateTime<Utc>,
    /// Command that created the layer
    pub created_by: Option<String>,
    /// If true, this is an empty layer
    pub empty_layer: bool,
}

/// Configuration of an image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    /// User to run as
    pub user: Option<String>,
    /// Working directory
    pub working_dir: Option<String>,
    /// Environment variables
    pub env: Vec<String>,
    /// Command to run
    pub cmd: Option<Vec<String>>,
    /// Entrypoint
    pub entrypoint: Option<Vec<String>>,
    /// Exposed ports
    pub exposed_ports: HashMap<String, HashMap<(), ()>>,
    /// Volumes
    pub volumes: HashMap<String, HashMap<(), ()>>,
    /// Labels
    pub labels: HashMap<String, String>,
    /// Architecture
    pub architecture: String,
    /// Operating system
    pub os: String,
}

impl Default for ImageConfig {
    fn default() -> Self {
        ImageConfig {
            user: None,
            working_dir: None,
            env: Vec::new(),
            cmd: None,
            entrypoint: None,
            exposed_ports: HashMap::new(),
            volumes: HashMap::new(),
            labels: HashMap::new(),
            architecture: "amd64".to_string(),
            os: "linux".to_string(),
        }
    }
}

/// ImageTag represents a tag of an image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageTag {
    /// Repository name
    pub repo: String,
    /// Tag name
    pub tag: String,
    /// Image ID
    pub image_id: String,
}

/// ImageReference represents a reference to an image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageReference {
    /// Repository name
    pub repo: String,
    /// Tag name
    pub tag: Option<String>,
    /// Digest
    pub digest: Option<String>,
}

impl ImageReference {
    /// Parse an image reference from a string
    pub fn parse(reference: &str) -> Option<Self> {
        // Handle digest
        if let Some(at_pos) = reference.find('@') {
            let repo = reference[..at_pos].to_string();
            let digest = reference[at_pos + 1..].to_string();
            return Some(ImageReference {
                repo,
                tag: None,
                digest: Some(digest),
            });
        }

        // Handle tag
        if let Some(colon_pos) = reference.find(':') {
            let repo = reference[..colon_pos].to_string();
            let tag = reference[colon_pos + 1..].to_string();
            return Some(ImageReference {
                repo,
                tag: Some(tag),
                digest: None,
            });
        }

        // Handle repo only
        Some(ImageReference {
            repo: reference.to_string(),
            tag: Some("latest".to_string()),
            digest: None,
        })
    }

    /// Convert to string
    pub fn to_string(&self) -> String {
        if let Some(digest) = &self.digest {
            format!("{}@{}", self.repo, digest)
        } else if let Some(tag) = &self.tag {
            format!("{}:{}", self.repo, tag)
        } else {
            self.repo.clone()
        }
    }
} 
