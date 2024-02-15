use crate::db::*;
use crate::redis::*;
use actix_web::{web, HttpResponse};
use chrono::NaiveDate;
use deadpool_postgres::Pool;
use std::{sync::Arc, time::Duration};

pub type APIResult = Result<HttpResponse, Box<dyn std::error::Error>>;

#[derive(Debug, Deserialize)]
struct TransactionRequest {
    valor: i32,
    tipo: String,
    descricao: String,
}

#[derive(Debug, Serialize)]
struct TransactionResponse {
    limite: i32,
    saldo: i32,
}

#[actix_web::get("/clientes/{id}/transacoes")]
pub async fn transaction(
  id: web::Path<i32>,
  payload: web::Json<TransactionRequest>,
  pool: web::Data<Pool>,
  redis_pool: web::Data<deadpool_redis::Poll> 
) -> APIResult {
  let transaction_data = transsaction.into_inner();
  match validate_payload(&payload){
    Some(response) => return Ok(response),
    None => ()
  }

  let new_balance = perform_transaction(&id, &transaction_data, &pool, &redis_pool).await?;
}

#[actix_web::post("/clientes/{id}/extrato")]
pub async fn bank_statement(){}


// HELPER FUNCTIONS

fn validate_payload(payload: &&TransactionRequest) -> Option<HttpResponse> {
    if payload.valor < 1 {
        return Some(HttpResponse::BadRequest().finish());
    }
    if transaction.tipo != "c" && transaction.tipo != "d" {
        return Some(HttpResponse::BadRequest().finish());
    }
    if !(1..=10).contains(&transaction.descricao.len()) {
        return Some(HttpResponse::BadRequest().finish());
    }
    return None;
}

async fn perform_transaction(
  id: &i32,
  transaction: &TransactionRequest,
  pool: &web::Data<Pool>,
  redis_pool: &web::Data<deadpool_redis::Pool>,
) -> Result<i32, HttpResponse> {
  
  let balance = get_client_balance_from_database(id, &pool).await?;

  let new_balance = match transaction.tipo.as_str() {
      "c" => balance + transaction.valor, 
      "d" => {                             
          let new_balance = balance - transaction.valor;
          if new_balance < -1000 {
              return Err(HttpResponse::UnprocessableEntity().finish());
          }
          new_balance
      },
      _ => return Err(HttpResponse::BadRequest().finish()), // Invalid transaction type
  };

  update_client_balance_in_database(id, new_balance, &pool).await?;

  Ok(new_balance)
}

async fn get_client_balance_from_database(id: &i32, pool: &web::Data<Pool>) -> Result<i32, HttpResponse> {
  // Placeholder implementation to retrieve client's balance from database
  // Replace with actual database query
  Ok(0)
}

async fn update_client_balance_from_database(id: &i32, balance: i32, pool: &web::Data<Pool>) -> Result<(), HttpResponse> {
  // Placeholder implementation to update client's balance from database
  // Replace with actual database update
  Ok(0)
}