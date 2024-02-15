use std::sync::Arc;

use actix_web::web;
use deadpool_postgres::Pool;
use serde::Deserialize;

pub type AsyncVoidResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
pub type QueueEvent = (String, web::Json<CriarClienteDTO>, Option<String>);
pub type AppQueue = deadqueue::unlimited::Queue<QueueEvent>;

#[derive(Deserialize)]
pub struct CriarClienteDTO {
  pub limite: f64,
  pub saldo_inicial: f64
}

pub async fn batch_insert(pool: Pool, queue: Arc<AppQueue>) {
  
}