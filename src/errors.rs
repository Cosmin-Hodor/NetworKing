use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Database connection failed: {0}")]
    DatabaseError(String),
    
    #[error("Network scanning error: {0}")]
    ScanningError(String),
    
    #[error("Geolocation service error: {0}")]
    GeolocationError(String),
}

// Implement conversion traits
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::ScanningError(err.to_string())
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(err: mongodb::error::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::GeolocationError(err.to_string())
    }
}

impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        AppError::ConfigError(err.to_string())
    }
}