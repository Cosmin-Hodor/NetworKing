use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerConfig {
    pub mongodb_uri: String,
    pub db_name: String,
    pub collection_name: String,
    pub scan_port: u16,
    pub start_ip: IpAddr,
    pub end_ip: IpAddr,
    pub scan_interval_ms: u64,
}

impl ScannerConfig {
    pub fn load() -> Result<Self, AppError> {
        // Load environment variables
        dotenv().ok();

        Ok(ScannerConfig {
            mongodb_uri: env::var("MONGODB_URI")
                .map_err(|e| AppError::ConfigError(e.to_string()))?,
            db_name: env::var("MONGO_DB_NAME")
                .unwrap_or_else(|_| "network_scan_db".to_string()),
            collection_name: env::var("MONGO_COLLECTION")
                .unwrap_or_else(|_| "accessible_ips".to_string()),
            scan_port: env::var("SCAN_PORT")
                .unwrap_or_else(|_| "11434".to_string())
                .parse()
                .map_err(|e| AppError::ConfigError(format!("Invalid port number: {}", e)))?,
            start_ip: IpAddr::from_str(&env::var("START_IP")
                .map_err(|e| AppError::ConfigError(e.to_string()))?)
                .map_err(|e| AppError::ConfigError(format!("Invalid start IP: {}", e)))?,
            end_ip: IpAddr::from_str(&env::var("END_IP")
                .map_err(|e| AppError::ConfigError(e.to_string()))?)
                .map_err(|e| AppError::ConfigError(format!("Invalid end IP: {}", e)))?,
            scan_interval_ms: env::var("SCAN_INTERVAL_MS")
                .unwrap_or_else(|_| "50".to_string())
                .parse()
                .map_err(|e| AppError::ConfigError(format!("Invalid scan interval: {}", e)))?,
        })
    }
}

// Utility functions for IP range handling
pub fn ip_to_u32(ip: IpAddr) -> u32 {
    match ip {
        IpAddr::V4(ipv4) => u32::from_be_bytes(ipv4.octets()),
        _ => panic!("Only IPv4 addresses are supported"),
    }
}

pub fn u32_to_ip(value: u32) -> IpAddr {
    IpAddr::V4(std::net::Ipv4Addr::from(value.to_be_bytes()))
}

pub struct IpRange {
    current: u32,
    end: u32,
}

impl IpRange {
    pub fn new(start: IpAddr, end: IpAddr) -> Self {
        let start_u32 = ip_to_u32(start);
        let end_u32 = ip_to_u32(end);
        
        IpRange {
            current: start_u32,
            end: end_u32,
        }
    }
}

impl Iterator for IpRange {
    type Item = IpAddr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            None
        } else {
            let current_ip = u32_to_ip(self.current);
            self.current += 1;
            Some(current_ip)
        }
    }
}