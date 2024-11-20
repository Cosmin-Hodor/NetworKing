use std::net::IpAddr;
use std::collections::HashMap;
use std::sync::Mutex;
use chrono::{DateTime, Utc};
use reqwest;
use log::{error};
use serde_json::Value;

pub struct GeolocationService {
    http_client: reqwest::Client,
    cache: Mutex<HashMap<String, (String, DateTime<Utc>)>>,
}

impl GeolocationService {
    pub fn new() -> Self {
        GeolocationService {
            http_client: reqwest::Client::new(),
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub async fn get_country_for_ip(&self, ip: IpAddr) -> Option<String> {
        let ip_str = ip.to_string();

        // Check cache first
        if let Some(cached_country) = self.get_cached_country(&ip_str) {
            return Some(cached_country);
        }

        // List of geolocation services
        let services = vec![
            format!("https://ipapi.co/{}/json/", ip_str),
            format!("https://ip-api.com/json/{}", ip_str),
        ];

        // Try each service
        for service_url in services {
            match self.fetch_geolocation(&service_url).await {
                Ok(Some(country)) => {
                    // Cache the result
                    self.cache_country(ip_str.clone(), country.clone());
                    return Some(country);
                },
                Ok(None) => continue,
                Err(e) => {
                    error!("Geolocation service error for {}: {}", ip_str, e);
                }
            }
        }

        None
    }

    fn get_cached_country(&self, ip: &str) -> Option<String> {
        let cache = self.cache.lock().unwrap();
        cache.get(ip)
            .filter(|(_, timestamp)| 
                timestamp.signed_duration_since(Utc::now()).num_days() < 30
            )
            .map(|(country, _)| country.clone())
    }

    fn cache_country(&self, ip: String, country: String) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(ip, (country, Utc::now()));
    }

    async fn fetch_geolocation(&self, url: &str) -> Result<Option<String>, reqwest::Error> {
        // Use async send
        let response = self.http_client
            .get(url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await?;

        let json: Value = response.json().await?;

        // Handle different API response formats
        let country_code = match url.contains("ipapi.co") {
            true => json.get("country_code").and_then(|c| c.as_str().map(|s| s.to_string())),
            false => json.get("countryCode").and_then(|c| c.as_str().map(|s| s.to_string()))
        };

        Ok(country_code)
    }
}