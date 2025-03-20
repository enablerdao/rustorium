use std::net::{TcpListener, SocketAddr};
use anyhow::Result;

/// 空きポートを見つける
pub fn find_available_port(start_port: u16) -> Result<u16> {
    let mut port = start_port;
    while port < 65535 {
        let addr = format!("127.0.0.1:{}", port).parse::<SocketAddr>()?;
        if TcpListener::bind(addr).is_ok() {
            return Ok(port);
        }
        port += 1;
    }
    anyhow::bail!("No available ports found")
}

/// 連続した空きポートを見つける
pub fn find_consecutive_ports(start_port: u16, count: u16) -> Result<Vec<u16>> {
    let mut port = start_port;
    while port < 65535 - count {
        let mut ports = Vec::new();
        let mut success = true;

        // count個の連続したポートをチェック
        for offset in 0..count {
            let addr = format!("127.0.0.1:{}", port + offset).parse::<SocketAddr>()?;
            if TcpListener::bind(addr).is_ok() {
                ports.push(port + offset);
            } else {
                success = false;
                break;
            }
        }

        if success {
            return Ok(ports);
        }

        port += count;
    }
    anyhow::bail!("No consecutive available ports found")
}