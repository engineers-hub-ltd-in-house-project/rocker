use rocker_core::errors::RockerError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

mod instruction;
mod parser;
mod stage;

pub use instruction::*;
pub use parser::*;
pub use stage::*;

/// Error type for Rockerfile parsing errors
#[derive(Error, Debug)]
pub enum RockerfileError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Invalid instruction: {0}")]
    InvalidInstruction(String),

    #[error("Missing argument for instruction: {0}")]
    MissingArgument(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("FROM instruction must be the first instruction in a Rockerfile")]
    FromNotFirst,

    #[error("Unknown instruction: {0}")]
    UnknownInstruction(String),
}

/// Result type for Rockerfile parsing
pub type Result<T> = std::result::Result<T, RockerfileError>;

/// Main function to parse a Rockerfile
pub fn parse_rockerfile<P: AsRef<Path>>(path: P) -> Result<Vec<Stage>> {
    let parser = RockerfileParser::new();
    parser.parse_file(path)
}

/// Build context for a Rockerfile build
#[derive(Debug, Clone)]
pub struct BuildContext {
    /// Root directory of the build context
    pub context_dir: PathBuf,
    /// Path to the Rockerfile
    pub rockerfile: PathBuf,
    /// Build arguments
    pub build_args: HashMap<String, String>,
    /// Labels to apply to the image
    pub labels: HashMap<String, String>,
    /// Target stage to build (if multi-stage build)
    pub target: Option<String>,
    /// Whether to use cache for the build
    pub no_cache: bool,
}

impl BuildContext {
    /// Create a new build context
    pub fn new<P: AsRef<Path>>(context_dir: P) -> Self {
        let context_dir = context_dir.as_ref().to_path_buf();
        let rockerfile = context_dir.join("Rockerfile");

        BuildContext {
            context_dir,
            rockerfile,
            build_args: HashMap::new(),
            labels: HashMap::new(),
            target: None,
            no_cache: false,
        }
    }

    /// Set the Rockerfile path
    pub fn with_rockerfile<P: AsRef<Path>>(mut self, rockerfile: P) -> Self {
        self.rockerfile = rockerfile.as_ref().to_path_buf();
        self
    }

    /// Add a build argument
    pub fn with_build_arg(mut self, key: String, value: String) -> Self {
        self.build_args.insert(key, value);
        self
    }

    /// Add a label
    pub fn with_label(mut self, key: String, value: String) -> Self {
        self.labels.insert(key, value);
        self
    }

    /// Set the target stage
    pub fn with_target(mut self, target: String) -> Self {
        self.target = Some(target);
        self
    }

    /// Set whether to use cache
    pub fn with_no_cache(mut self, no_cache: bool) -> Self {
        self.no_cache = no_cache;
        self
    }

    /// Check if the build context is valid
    pub fn validate(&self) -> Result<()> {
        if !self.context_dir.exists() {
            return Err(RockerfileError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Context directory not found: {:?}", self.context_dir),
            )));
        }

        if !self.rockerfile.exists() {
            return Err(RockerfileError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Rockerfile not found: {:?}", self.rockerfile),
            )));
        }

        Ok(())
    }
} 
