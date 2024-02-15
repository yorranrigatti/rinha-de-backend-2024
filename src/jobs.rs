use crate::db::*;
use deadpool_postgres::Pool;
use std::{sync::Arc, time::Duration};


pub async fn db_warmup() {
  println!("warming up...");
  tokio::time::sleep(Duration::from_secs(3)).await;
  let http_client = reqwest::Client::new();
  let nginx_url = "http://localhost:9999/clientes";
  let mount_body = |id: u16, limite: u32| {
      format!("{{\"id\":{},\"limite\":{},\"saldo_inicial\":0}}", id, limite)
  };

  let mut f = vec![];

  for i in 1..=5 {
      let (id, limite) = match i {
          1 => (1, 100000),
          2 => (2, 80000),
          3 => (3, 1000000),
          4 => (4, 10000000),
          5 => (5, 500000),
          _ => unreachable!(),
      };

      f.push(
          http_client
              .post(nginx_url)
              .body(mount_body(id, limite))
              .header("Content-Type", "application/json")
              .send(),
      );
  }

  futures::future::join_all(f).await;
  println!("warmup finished");
}

pub async fn db_clean_warmup(pool: Pool) {
  println!("cleaning warmup data...");
  tokio::time::sleep(Duration::from_secs(3)).await;
  pool.get().await.unwrap().execute("DELETE FROM clientes", &[]).await.unwrap();
}

pub async fn db_flush_queue(pool_async: Pool, queue_async: Arc<AppQueue>) {
 println!("queue flush job started (loop every 2 seconds)");
 loop {
  tokio::time::sleep(Duration::from_secs(2)).await;
  let queue = queue_async.clone();
  if queue.len() == 0 {
    continue;
  }
  batch_insert(pool_async.clone(), queue).await;
 } 
}