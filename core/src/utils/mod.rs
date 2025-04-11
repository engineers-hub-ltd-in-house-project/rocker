use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::Path;

mod id;
pub use id::*;

/// Calculate the SHA256 hash of a file
pub fn calculate_file_hash<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();
    Ok(format!("sha256:{:x}", hash))
}

/// Calculate the SHA256 hash of a string
pub fn calculate_string_hash(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    let hash = hasher.finalize();
    format!("sha256:{:x}", hash)
}

/// Format a size in bytes to a human-readable string
pub fn format_size(size: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    
    if size == 0 {
        return "0 B".to_string();
    }
    
    let index = (size as f64).log(1024.0).floor() as usize;
    let index = std::cmp::min(index, UNITS.len() - 1);
    
    let size = size as f64 / 1024f64.powi(index as i32);
    
    format!("{:.2} {}", size, UNITS[index])
}

/// Format a duration in seconds to a human-readable string
pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        return format!("{}s", seconds);
    }
    
    let minutes = seconds / 60;
    if minutes < 60 {
        return format!("{}m{}s", minutes, seconds % 60);
    }
    
    let hours = minutes / 60;
    if hours < 24 {
        return format!("{}h{}m", hours, minutes % 60);
    }
    
    let days = hours / 24;
    format!("{}d{}h", days, hours % 24)
}

/// Parse a string into a memory size in bytes
pub fn parse_memory_size(s: &str) -> Result<u64, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("Empty string".to_string());
    }
    
    let num_str = s
        .chars()
        .take_while(|c| c.is_digit(10) || *c == '.')
        .collect::<String>();
        
    let unit_str = s
        .chars()
        .skip(num_str.len())
        .collect::<String>()
        .to_lowercase();
        
    let num = num_str.parse::<f64>().map_err(|e| e.to_string())?;
    
    let multiplier: u64 = match unit_str.as_str() {
        "" | "b" => 1,
        "k" | "kb" => 1024,
        "m" | "mb" => 1024 * 1024,
        "g" | "gb" => 1024 * 1024 * 1024,
        "t" | "tb" => 1024u64 * 1024 * 1024 * 1024,
        _ => return Err(format!("Unknown unit: {}", unit_str)),
    };
    
    Ok((num * multiplier as f64) as u64)
}

/// Generate a random port number
pub fn random_port() -> u16 {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
        
    let mut num = (now & 0xFFFF) as u16;
    
    // Ensure the port is in the dynamic range (49152-65535)
    if num < 49152 {
        num += 49152;
    }
    
    num
}

/// Check if a port is available
pub fn is_port_available(port: u16) -> bool {
    use std::net::{SocketAddrV4, TcpListener};
    use std::str::FromStr;
    
    let addr = SocketAddrV4::from_str(&format!("127.0.0.1:{}", port))
        .expect("Failed to parse socket address");
        
    TcpListener::bind(addr).is_ok()
} 
