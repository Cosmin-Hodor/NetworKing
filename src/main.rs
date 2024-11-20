mod models;
mod errors;
mod config;
mod scanner;
mod database;
mod geolocation;
mod logging;
mod retry;

use log::{info, error};
use tokio::time::{sleep, Duration};
use crate::errors::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Setup logging
    logging::setup_logging();

    // Load configuration with error handling
    let config = match retry::retry_operation(
        || async {
            config::ScannerConfig::load()
        },
        3,
        "Configuration Loading"
    ).await {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load configuration: {:?}", e);
            return Err(e);
        }
    };

    // Initialize database manager with error handling
    let db_manager = match retry::retry_operation(
        || async {
            database::DatabaseManager::new(&config).await
        },
        3,
        "Database Connection"
    ).await {
        Ok(manager) => manager,
        Err(e) => {
            error!("Failed to connect to database: {:?}", e);
            return Err(e);
        }
    };

    // Create network scanner
    let scanner = scanner::NetworkScanner::new(config.clone());

    // Continuous scanning loop with error handling
    loop {
        // Perform network scan
        let scan_results = match retry::retry_operation(
            || async {
                scanner.scan_network().await
            },
            3,
            "Network Scanning"
        ).await {
            Ok(results) => results,
            Err(e) => {
                error!("Network scanning failed: {:?}", e);
                // Wait before retrying
                sleep(Duration::from_secs(60)).await;
                continue;
            }
        };

        // Store results if not empty
        if !scan_results.is_empty() {
            match retry::retry_operation(
                || async {
                    db_manager.store_results(&scan_results).await
                },
                3,
                "Results Storage"
            ).await {
                Ok(_) => {
                    info!("Scan completed successfully. Discovered {} accessible IPs", scan_results.len());
                },
                Err(e) => {
                    error!("Failed to store scan results: {:?}", e);
                }
            }
        }

        // Wait before next scan
        info!("Waiting before next scan...");
        sleep(Duration::from_secs(config.scan_interval_ms)).await;
    }
}