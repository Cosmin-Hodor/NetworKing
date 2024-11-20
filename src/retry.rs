use std::time::Duration;
use log::{warn, info};
use crate::errors::AppError;

pub async fn retry_operation<F, Fut, T, E>(
    operation: F,
    max_retries: u32,
    operation_name: &str
) -> Result<T, AppError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut retry_count = 0;
    
    loop {
        match operation().await {
            Ok(result) => {
                if retry_count > 0 {
                    info!("{} succeeded after {} retries", operation_name, retry_count);
                }
                return Ok(result);
            },
            Err(e) => {
                retry_count += 1;
                
                if retry_count >= max_retries {
                    warn!("{} failed after {} retries", operation_name, retry_count);
                    return Err(AppError::ScanningError(format!("{:?}", e)));
                }
                
                warn!(
                    "{} attempt {} failed. Error: {:?}. Retrying...", 
                    operation_name, 
                    retry_count, 
                    e
                );
                
                // Exponential backoff
                let delay = Duration::from_secs((2u64.pow(retry_count)).min(60));
                tokio::time::sleep(delay).await;
            }
        }
    }
}