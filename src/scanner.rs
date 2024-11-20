use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;
use log::{info, error};
use anyhow::Result;
use chrono::Utc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::models::IPScanResult;
use crate::config::{ScannerConfig, IpRange, ip_to_u32};
use crate::geolocation::GeolocationService;
use crate::errors::AppError;

pub struct NetworkScanner {
    config: ScannerConfig,
    geolocation_service: GeolocationService,
    progress: Arc<AtomicUsize>,
}

impl NetworkScanner {
    pub fn new(config: ScannerConfig) -> Self {
        NetworkScanner { 
            config, 
            geolocation_service: GeolocationService::new(),
            progress: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub async fn scan_network(&self) -> Result<Vec<IPScanResult>, AppError> {
      // Reset progress
      self.progress.store(0, Ordering::SeqCst);
  
      // Create IP range iterator
      let ip_range = IpRange::new(self.config.start_ip, self.config.end_ip);
      let total_ips = (ip_to_u32(self.config.end_ip) - ip_to_u32(self.config.start_ip) + 1) as usize;
  
      // Parallel scanning of IP addresses
      let mut results: Vec<IPScanResult> = Vec::new();
  
      for ip in ip_range {
          // Update progress
          let current = self.progress.fetch_add(1, Ordering::SeqCst);
          
          // Periodically log progress
          if current % 100 == 0 {
              info!(
                  "Scanning progress: {:.2}% ({}/{})", 
                  (current as f32 / total_ips as f32) * 100.0, 
                  current,
                  total_ips
              );
          }

  
          // Simulate a small delay between scans to reduce network pressure
          std::thread::sleep(Duration::from_millis(std::cmp::min(self.config.scan_interval_ms, 200)));
          
          match self.scan_single_ip(ip, self.config.scan_port).await {
              Some(result) => {
                  info!("Accessible IP found: {} on port {}", result.ip, result.port);
                  results.push(result);
              },
              None => continue
          }
      }
  
      info!(
          "Scan completed. Scanned {} IPs, found {} accessible", 
          total_ips, 
          results.len()
      );
  
      Ok(results)
  }

    async fn scan_single_ip(&self, ip: IpAddr, port: u16) -> Option<IPScanResult> {
        let socket_addr = SocketAddr::new(ip, port);

        // Set a shorter timeout for faster scanning
        match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(1)) {
            Ok(_) => {
                // Get country asynchronously
                println!("Found {}", ip);

                match self.geolocation_service
                .get_country_for_ip(ip).await {
                    Some(country) => {
                        Some(IPScanResult {
                            ip: ip.to_string(),
                            port,
                            country: Some(country),
                            last_scanned: Utc::now(),
                        })
                    },
                    None => {
                        error!("Failed to get geolocation for IP: {}", ip);
                        Some(IPScanResult {
                            ip: ip.to_string(),
                            port,
                            country: None,
                            last_scanned: Utc::now(),
                        })
                    }
                }
            },
            Err(e) => {
                // Optionally log connection failures at a lower frequency
                if rand::random::<u8>() % 100 == 0 {
                    error!("Connection to {} failed: {}", ip, e);
                }
                None
            }
        }
    }
}