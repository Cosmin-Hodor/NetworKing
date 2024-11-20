use mongodb::{
  Client, 
  Collection,
  options::{ClientOptions, ServerApi, ServerApiVersion},
  bson::Document,
};
use log::{info, error};
use crate::config::ScannerConfig;
use crate::models::IPScanResult;
use crate::errors::AppError;

pub struct DatabaseManager {
  client: Client,
  config: ScannerConfig,
}

impl DatabaseManager {
  pub async fn new(config: &ScannerConfig) -> Result<Self, AppError> {
      // Configure client options
      let mut client_options = ClientOptions::parse(&config.mongodb_uri)
          .await
          .map_err(|e| {
              error!("Failed to parse MongoDB URI: {}", e);
              AppError::DatabaseError(e.to_string())
          })?;

      // Set the server API version
      let server_api = ServerApi::builder()
          .version(ServerApiVersion::V1)
          .build();
      client_options.server_api = Some(server_api);

      // Create the client
      let client = Client::with_options(client_options)
          .map_err(|e| {
              error!("Failed to create MongoDB client: {}", e);
              AppError::DatabaseError(e.to_string())
          })?;

      // Verify connection
      client.database("admin").run_command(mongodb::bson::doc! {"ping": 1}, None)
          .await
          .map_err(|e| {
              error!("Failed to ping MongoDB: {}", e);
              AppError::DatabaseError(e.to_string())
          })?;

      info!("Successfully connected to MongoDB");

      Ok(DatabaseManager {
          client,
          config: config.clone(),
      })
  }

  pub async fn store_results(&self, results: &[IPScanResult]) -> Result<(), AppError> {
      let db = self.client.database(&self.config.db_name);
      let collection: Collection<Document> = db.collection(&self.config.collection_name);

      let mut successful_inserts = 0;
      let mut failed_inserts = 0;

      for result in results {
          let filter = mongodb::bson::doc! {
              "ip": &result.ip,
              "port": result.port as i32
          };

          let update = mongodb::bson::doc! {
              "$set": mongodb::bson::to_document(result)
                  .map_err(|e| {
                      error!("Failed to convert result to document: {}", e);
                      AppError::DatabaseError(e.to_string())
                  })?
          };

          match collection
              .update_one(
                  filter, 
                  update, 
                  Some(mongodb::options::UpdateOptions::builder().upsert(true).build())
              )
              .await {
                  Ok(_) => successful_inserts += 1,
                  Err(e) => {
                      error!("Failed to insert/update record: {}", e);
                      failed_inserts += 1;
                  }
              }
      }

      info!(
          "Database operation results: {} successful, {} failed, total {} records", 
          successful_inserts, 
          failed_inserts, 
          results.len()
      );

      if failed_inserts > 0 {
          Err(AppError::DatabaseError(format!(
              "{} out of {} records failed to insert",
              failed_inserts,
              results.len()
          )))
      } else {
          Ok(())
      }
  }
}