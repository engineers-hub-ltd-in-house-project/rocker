use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Instruction represents a Rockerfile instruction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instruction {
    /// FROM instruction
    From {
        /// Image name
        image: String,
        /// Stage name (for multi-stage builds)
        as_name: Option<String>,
    },
    /// RUN instruction
    Run {
        /// Command to run
        command: String,
    },
    /// COPY instruction
    Copy {
        /// Source path(s)
        sources: Vec<String>,
        /// Destination path
        destination: String,
        /// Source stage (for multi-stage builds)
        from: Option<String>,
        /// Change ownership
        chown: Option<String>,
        /// Change permission
        chmod: Option<String>,
    },
    /// ADD instruction
    Add {
        /// Source path(s)
        sources: Vec<String>,
        /// Destination path
        destination: String,
        /// Change ownership
        chown: Option<String>,
        /// Change permission
        chmod: Option<String>,
    },
    /// WORKDIR instruction
    Workdir {
        /// Working directory
        path: String,
    },
    /// ENV instruction
    Env {
        /// Environment variables
        variables: HashMap<String, String>,
    },
    /// ARG instruction
    Arg {
        /// Argument name
        name: String,
        /// Default value
        default_value: Option<String>,
    },
    /// EXPOSE instruction
    Expose {
        /// Ports to expose
        ports: Vec<u16>,
        /// Protocol (tcp or udp)
        protocol: Option<String>,
    },
    /// LABEL instruction
    Label {
        /// Labels
        labels: HashMap<String, String>,
    },
    /// USER instruction
    User {
        /// User name or ID
        user: String,
        /// Group name or ID
        group: Option<String>,
    },
    /// VOLUME instruction
    Volume {
        /// Volumes
        paths: Vec<String>,
    },
    /// CMD instruction
    Cmd {
        /// Command
        command: Vec<String>,
    },
    /// ENTRYPOINT instruction
    Entrypoint {
        /// Entrypoint command
        command: Vec<String>,
    },
    /// HEALTHCHECK instruction
    Healthcheck {
        /// Command to run
        command: Vec<String>,
        /// Interval between health checks
        interval: Option<String>,
        /// Timeout for a health check
        timeout: Option<String>,
        /// Number of retries before considering the container unhealthy
        retries: Option<u32>,
        /// Start period before health checks count towards the retry count
        start_period: Option<String>,
    },
    /// SHELL instruction
    Shell {
        /// Shell to use
        shell: Vec<String>,
    },
    /// STOPSIGNAL instruction
    StopSignal {
        /// Signal to stop the container
        signal: String,
    },
    /// ONBUILD instruction
    OnBuild {
        /// Instruction to run when this image is used as a base image
        instruction: Box<Instruction>,
    },
}

impl Instruction {
    /// Returns true if this instruction is a FROM instruction
    pub fn is_from(&self) -> bool {
        matches!(self, Instruction::From { .. })
    }

    /// Returns the name of the instruction
    pub fn name(&self) -> &'static str {
        match self {
            Instruction::From { .. } => "FROM",
            Instruction::Run { .. } => "RUN",
            Instruction::Copy { .. } => "COPY",
            Instruction::Add { .. } => "ADD",
            Instruction::Workdir { .. } => "WORKDIR",
            Instruction::Env { .. } => "ENV",
            Instruction::Arg { .. } => "ARG",
            Instruction::Expose { .. } => "EXPOSE",
            Instruction::Label { .. } => "LABEL",
            Instruction::User { .. } => "USER",
            Instruction::Volume { .. } => "VOLUME",
            Instruction::Cmd { .. } => "CMD",
            Instruction::Entrypoint { .. } => "ENTRYPOINT",
            Instruction::Healthcheck { .. } => "HEALTHCHECK",
            Instruction::Shell { .. } => "SHELL",
            Instruction::StopSignal { .. } => "STOPSIGNAL",
            Instruction::OnBuild { .. } => "ONBUILD",
        }
    }

    /// Returns a string representation of the instruction
    pub fn to_string(&self) -> String {
        match self {
            Instruction::From { image, as_name } => {
                if let Some(name) = as_name {
                    format!("FROM {} AS {}", image, name)
                } else {
                    format!("FROM {}", image)
                }
            }
            Instruction::Run { command } => {
                format!("RUN {}", command)
            }
            Instruction::Copy {
                sources,
                destination,
                from,
                chown,
                chmod,
            } => {
                let mut options = Vec::new();
                if let Some(f) = from {
                    options.push(format!("--from={}", f));
                }
                if let Some(c) = chown {
                    options.push(format!("--chown={}", c));
                }
                if let Some(c) = chmod {
                    options.push(format!("--chmod={}", c));
                }

                let options_str = if !options.is_empty() {
                    format!("{} ", options.join(" "))
                } else {
                    String::new()
                };

                let sources_str = sources.join(" ");
                format!("COPY {}{} {}", options_str, sources_str, destination)
            }
            Instruction::Add {
                sources,
                destination,
                chown,
                chmod,
            } => {
                let mut options = Vec::new();
                if let Some(c) = chown {
                    options.push(format!("--chown={}", c));
                }
                if let Some(c) = chmod {
                    options.push(format!("--chmod={}", c));
                }

                let options_str = if !options.is_empty() {
                    format!("{} ", options.join(" "))
                } else {
                    String::new()
                };

                let sources_str = sources.join(" ");
                format!("ADD {}{} {}", options_str, sources_str, destination)
            }
            Instruction::Workdir { path } => {
                format!("WORKDIR {}", path)
            }
            Instruction::Env { variables } => {
                let vars = variables
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("ENV {}", vars)
            }
            Instruction::Arg { name, default_value } => {
                if let Some(value) = default_value {
                    format!("ARG {}={}", name, value)
                } else {
                    format!("ARG {}", name)
                }
            }
            Instruction::Expose { ports, protocol } => {
                let proto = protocol.as_deref().unwrap_or("tcp");
                let ports_str = ports
                    .iter()
                    .map(|p| format!("{}/{}", p, proto))
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("EXPOSE {}", ports_str)
            }
            Instruction::Label { labels } => {
                let label_str = labels
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("LABEL {}", label_str)
            }
            Instruction::User { user, group } => {
                if let Some(g) = group {
                    format!("USER {}:{}", user, g)
                } else {
                    format!("USER {}", user)
                }
            }
            Instruction::Volume { paths } => {
                let paths_str = paths.join(" ");
                format!("VOLUME {}", paths_str)
            }
            Instruction::Cmd { command } => {
                let cmd_str = serde_json::to_string(&command).unwrap();
                format!("CMD {}", cmd_str)
            }
            Instruction::Entrypoint { command } => {
                let cmd_str = serde_json::to_string(&command).unwrap();
                format!("ENTRYPOINT {}", cmd_str)
            }
            Instruction::Healthcheck {
                command,
                interval,
                timeout,
                retries,
                start_period,
            } => {
                let mut options = Vec::new();
                if let Some(i) = interval {
                    options.push(format!("--interval={}", i));
                }
                if let Some(t) = timeout {
                    options.push(format!("--timeout={}", t));
                }
                if let Some(r) = retries {
                    options.push(format!("--retries={}", r));
                }
                if let Some(s) = start_period {
                    options.push(format!("--start-period={}", s));
                }

                let options_str = if !options.is_empty() {
                    format!("{} ", options.join(" "))
                } else {
                    String::new()
                };

                let cmd_str = serde_json::to_string(&command).unwrap();
                format!("HEALTHCHECK {}CMD {}", options_str, cmd_str)
            }
            Instruction::Shell { shell } => {
                let shell_str = serde_json::to_string(&shell).unwrap();
                format!("SHELL {}", shell_str)
            }
            Instruction::StopSignal { signal } => {
                format!("STOPSIGNAL {}", signal)
            }
            Instruction::OnBuild { instruction } => {
                format!("ONBUILD {}", instruction.to_string())
            }
        }
    }
} 
