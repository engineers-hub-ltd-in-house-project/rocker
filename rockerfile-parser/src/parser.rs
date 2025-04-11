use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs;

use crate::{Instruction, Result, RockerfileError, Stage};

#[derive(Debug)]
pub struct RockerfileParser {
    stages: Vec<Stage>,
    current_stage: usize,
}

impl RockerfileParser {
    pub fn new() -> Self {
        RockerfileParser {
            stages: vec![Stage::new(None)],
            current_stage: 0,
        }
    }

    pub fn parse_file<P: AsRef<Path>>(&mut self, path: P) -> Result<&[Stage]> {
        let content = fs::read_to_string(path)?;
        self.parse_content(&content)?;
        Ok(&self.stages)
    }

    pub fn parse_content(&mut self, content: &str) -> Result<&[Stage]> {
        // 空の行やコメントを削除
        let lines: Vec<&str> = content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .collect();

        // 行を処理
        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];
            let mut parts = line.splitn(2, ' ');
            
            let instruction = parts.next().ok_or_else(|| RockerfileError::Parse("Invalid instruction".to_string()))?;
            let args = parts.next().unwrap_or("");
            
            // 行継続をサポート (\で終わる行)
            let mut full_args = args.to_string();
            while full_args.ends_with('\\') && i + 1 < lines.len() {
                full_args.pop(); // バックスラッシュを削除
                i += 1;
                full_args.push_str(lines[i].trim());
            }
            
            match instruction.to_uppercase().as_str() {
                "FROM" => self.parse_from(&full_args)?,
                "RUN" => self.parse_run(&full_args)?,
                "COPY" => self.parse_copy(&full_args)?,
                "WORKDIR" => self.parse_workdir(&full_args)?,
                "ENV" => self.parse_env(&full_args)?,
                "EXPOSE" => self.parse_expose(&full_args)?,
                "VOLUME" => self.parse_volume(&full_args)?,
                "CMD" => self.parse_cmd(&full_args)?,
                "ENTRYPOINT" => self.parse_entrypoint(&full_args)?,
                "LABEL" => self.parse_label(&full_args)?,
                "USER" => self.parse_user(&full_args)?,
                "ARG" => self.parse_arg(&full_args)?,
                _ => return Err(RockerfileError::UnknownInstruction(instruction.to_string())),
            }
            
            i += 1;
        }
        
        Ok(&self.stages)
    }

    fn parse_from(&mut self, args: &str) -> Result<()> {
        // FROMで新しいステージが始まる
        let mut parts = args.splitn(2, " AS ");
        let image = parts.next().ok_or_else(|| RockerfileError::InvalidInstruction("Invalid FROM instruction".to_string()))?.trim();
        let stage_name = parts.next().map(|s| s.trim().to_string());
        
        // 二つ目以降のステージの場合
        if !self.stages[self.current_stage].instructions.is_empty() {
            self.stages.push(Stage::new(stage_name));
            self.current_stage = self.stages.len() - 1;
        } else {
            // 最初のステージの場合
            self.stages[self.current_stage].name = stage_name;
        }
        
        self.stages[self.current_stage].add_instruction(Instruction::From {
            image: image.to_string(),
            as_name: parts.next().map(|s| s.trim().to_string()),
        });
            
        Ok(())
    }

    fn parse_run(&mut self, args: &str) -> Result<()> {
        self.stages[self.current_stage].add_instruction(Instruction::Run {
            command: args.to_string(),
        });
        Ok(())
    }

    fn parse_copy(&mut self, args: &str) -> Result<()> {
        // --fromオプションの解析をサポート
        let mut from = None;
        let mut chown = None;
        let mut chmod = None;
        let mut copy_args = args;
        
        // オプションの解析
        while copy_args.starts_with("--") {
            let parts: Vec<&str> = copy_args.splitn(2, ' ').collect();
            if parts.len() < 2 {
                return Err(RockerfileError::InvalidInstruction("Invalid COPY instruction".to_string()));
            }
            
            let opt = parts[0].trim();
            if opt.starts_with("--from=") {
                from = Some(opt[7..].to_string());
            } else if opt.starts_with("--chown=") {
                chown = Some(opt[8..].to_string());
            } else if opt.starts_with("--chmod=") {
                chmod = Some(opt[8..].to_string());
            } else {
                return Err(RockerfileError::InvalidInstruction(format!("Unknown option: {}", opt)));
            }
            
            copy_args = parts[1].trim();
        }
        
        // src と dest の解析
        let parts: Vec<&str> = copy_args.rsplitn(2, ' ').collect();
        if parts.len() < 2 {
            return Err(RockerfileError::InvalidInstruction("Invalid COPY instruction".to_string()));
        }
        
        let dest = parts[0].trim();
        let sources: Vec<String> = parts[1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        self.stages[self.current_stage].add_instruction(Instruction::Copy {
            sources,
            destination: dest.to_string(),
            from,
            chown,
            chmod,
        });
            
        Ok(())
    }

    fn parse_workdir(&mut self, args: &str) -> Result<()> {
        self.stages[self.current_stage].add_instruction(Instruction::Workdir {
            path: args.to_string(),
        });
        Ok(())
    }

    fn parse_env(&mut self, args: &str) -> Result<()> {
        let mut variables = HashMap::new();
        
        if args.contains('=') {
            // ENV KEY1=value1 KEY2=value2 形式
            let mut remaining = args;
            
            while !remaining.is_empty() {
                let key_end = remaining.find('=').ok_or_else(|| {
                    RockerfileError::InvalidInstruction("Invalid ENV format".to_string())
                })?;
                
                let key = remaining[..key_end].trim().to_string();
                remaining = &remaining[key_end + 1..];
                
                let value_end = if remaining.starts_with('"') {
                    // クォートされた値
                    let mut pos = 1;
                    let mut escaped = false;
                    
                    while pos < remaining.len() {
                        let c = remaining.chars().nth(pos).unwrap();
                        
                        if escaped {
                            escaped = false;
                        } else if c == '\\' {
                            escaped = true;
                        } else if c == '"' {
                            break;
                        }
                        
                        pos += 1;
                    }
                    
                    if pos >= remaining.len() {
                        return Err(RockerfileError::InvalidInstruction("Unterminated quote in ENV".to_string()));
                    }
                    
                    pos + 1
                } else {
                    // 空白で区切られた値
                    remaining.find(' ').unwrap_or(remaining.len())
                };
                
                let value = remaining[..value_end].trim();
                let value = if value.starts_with('"') && value.ends_with('"') {
                    // クォートを取り除く
                    value[1..value.len() - 1].to_string()
                } else {
                    value.to_string()
                };
                
                variables.insert(key, value);
                
                if value_end < remaining.len() {
                    remaining = &remaining[value_end..].trim_start();
                } else {
                    break;
                }
            }
        } else {
            // ENV KEY value 形式
            let parts: Vec<&str> = args.splitn(2, ' ').collect();
            if parts.len() == 2 {
                variables.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
        
        self.stages[self.current_stage].add_instruction(Instruction::Env { variables });
            
        Ok(())
    }

    fn parse_expose(&mut self, args: &str) -> Result<()> {
        let mut ports = Vec::new();
        let mut protocol = None;
        
        for port_str in args.split_whitespace() {
            if let Some(proto_idx) = port_str.find('/') {
                let port_part = &port_str[..proto_idx];
                let proto_part = &port_str[proto_idx + 1..];
                
                let port = port_part.parse::<u16>().map_err(|_| {
                    RockerfileError::InvalidInstruction(format!("Invalid port: {}", port_part))
                })?;
                
                ports.push(port);
                
                if protocol.is_none() {
                    protocol = Some(proto_part.to_string());
                } else if protocol.as_ref().unwrap() != proto_part {
                    return Err(RockerfileError::InvalidInstruction(
                        "Mixed protocols in EXPOSE".to_string()
                    ));
                }
            } else {
                let port = port_str.parse::<u16>().map_err(|_| {
                    RockerfileError::InvalidInstruction(format!("Invalid port: {}", port_str))
                })?;
                
                ports.push(port);
            }
        }
        
        self.stages[self.current_stage].add_instruction(Instruction::Expose {
            ports,
            protocol,
        });
            
        Ok(())
    }

    fn parse_volume(&mut self, args: &str) -> Result<()> {
        // JSONフォーマットサポート: VOLUME ["/var/log", "/data"]
        if args.starts_with('[') && args.ends_with(']') {
            let json_volumes: Vec<String> = serde_json::from_str(args)?;
            self.stages[self.current_stage].add_instruction(Instruction::Volume {
                paths: json_volumes,
            });
        } else {
            // スペース区切り形式: VOLUME /var/log /data
            let volumes: Vec<String> = args
                .split_whitespace()
                .map(|v| v.to_string())
                .collect();
                
            self.stages[self.current_stage].add_instruction(Instruction::Volume {
                paths: volumes,
            });
        }
        
        Ok(())
    }

    fn parse_cmd(&mut self, args: &str) -> Result<()> {
        // JSONフォーマットサポート: CMD ["executable","param1","param2"]
        if args.starts_with('[') && args.ends_with(']') {
            let json_cmd: Vec<String> = serde_json::from_str(args)?;
            self.stages[self.current_stage].add_instruction(Instruction::Cmd {
                command: json_cmd,
            });
        } else {
            // シェル形式: CMD executable param1 param2
            // シェル形式はシェルによって実行される
            let command = vec![args.to_string()];
            self.stages[self.current_stage].add_instruction(Instruction::Cmd {
                command,
            });
        }
        
        Ok(())
    }

    fn parse_entrypoint(&mut self, args: &str) -> Result<()> {
        // JSONフォーマットサポート: ENTRYPOINT ["executable","param1"]
        if args.starts_with('[') && args.ends_with(']') {
            let json_entrypoint: Vec<String> = serde_json::from_str(args)?;
            self.stages[self.current_stage].add_instruction(Instruction::Entrypoint {
                command: json_entrypoint,
            });
        } else {
            // シェル形式: ENTRYPOINT executable param1 param2
            let command = vec![args.to_string()];
            self.stages[self.current_stage].add_instruction(Instruction::Entrypoint {
                command,
            });
        }
        
        Ok(())
    }

    fn parse_label(&mut self, args: &str) -> Result<()> {
        let mut labels = HashMap::new();
        
        // LABELの解析
        let mut remaining = args;
        
        while !remaining.is_empty() {
            // キーの解析
            let mut key_end = remaining.find('=').unwrap_or(remaining.len());
            let space_pos = remaining.find(' ').unwrap_or(remaining.len());
            
            if space_pos < key_end {
                // "LABEL key value" 形式
                let key = remaining[..space_pos].trim();
                remaining = &remaining[space_pos..].trim_start();
                
                // 値の取得
                let value_end = remaining.find(' ').unwrap_or(remaining.len());
                let value = remaining[..value_end].trim();
                
                labels.insert(key.to_string(), value.to_string());
                
                if value_end < remaining.len() {
                    remaining = &remaining[value_end..].trim_start();
                } else {
                    break;
                }
            } else {
                // "LABEL key=value" 形式
                let key = remaining[..key_end].trim();
                remaining = &remaining[key_end + 1..];
                
                // 値の解析
                let value_end = if remaining.starts_with('"') {
                    // クォートされた値
                    let mut pos = 1;
                    let mut escaped = false;
                    
                    while pos < remaining.len() {
                        let c = remaining.chars().nth(pos).unwrap();
                        
                        if escaped {
                            escaped = false;
                        } else if c == '\\' {
                            escaped = true;
                        } else if c == '"' {
                            break;
                        }
                        
                        pos += 1;
                    }
                    
                    if pos >= remaining.len() {
                        return Err(RockerfileError::InvalidInstruction("Unterminated quote in LABEL".to_string()));
                    }
                    
                    pos + 1
                } else {
                    // 空白で区切られた値
                    remaining.find(' ').unwrap_or(remaining.len())
                };
                
                let value = remaining[..value_end].trim();
                let value = if value.starts_with('"') && value.ends_with('"') {
                    // クォートを取り除く
                    value[1..value.len() - 1].to_string()
                } else {
                    value.to_string()
                };
                
                labels.insert(key.to_string(), value);
                
                if value_end < remaining.len() {
                    remaining = &remaining[value_end..].trim_start();
                } else {
                    break;
                }
            }
        }
        
        self.stages[self.current_stage].add_instruction(Instruction::Label { labels });
        
        Ok(())
    }

    fn parse_user(&mut self, args: &str) -> Result<()> {
        if let Some(colon_pos) = args.find(':') {
            let user = &args[0..colon_pos];
            let group = &args[colon_pos + 1..];
            
            self.stages[self.current_stage].add_instruction(Instruction::User {
                user: user.to_string(),
                group: Some(group.to_string()),
            });
        } else {
            self.stages[self.current_stage].add_instruction(Instruction::User {
                user: args.to_string(),
                group: None,
            });
        }
        
        Ok(())
    }

    fn parse_arg(&mut self, args: &str) -> Result<()> {
        if let Some(equals_pos) = args.find('=') {
            let name = &args[0..equals_pos];
            let default_value = &args[equals_pos + 1..];
            
            self.stages[self.current_stage].add_instruction(Instruction::Arg {
                name: name.to_string(),
                default_value: Some(default_value.to_string()),
            });
        } else {
            self.stages[self.current_stage].add_instruction(Instruction::Arg {
                name: args.to_string(),
                default_value: None,
            });
        }
        
        Ok(())
    }
} 
