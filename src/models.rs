use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IPScanResult {
    pub ip: String,
    pub port: u16,
    pub country: Option<String>,
    pub last_scanned: DateTime<Utc>,
}