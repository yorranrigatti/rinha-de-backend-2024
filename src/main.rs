use deadpool_postgres::{Config, PoolConfig, Runtime, Timeouts};
use deadpool_redis::{ConnectionAddr, ConnectionInfo, RedisConnectionInfo};
use std::{env, sync::Arc, time::Duration};
use tokio_postgres::NoTls;

mod db;
use db::*;

mod jobs;
use jobs::*;

#[tokio::main]
async fn main() -> AsyncVoidResult {
    let mut cfg: Config = Config::new();
    cfg.host = Some(env::var("DB_HOST").unwrap_or("localhost".into()).to_string());
    cfg.port = Some(env::var("DB_PORT").unwrap_or("5432".into()).parse()?);
    cfg.dbname = Some(env::var("DB_NAME").unwrap_or("rinhadb".into()));
    cfg.user = Some(env::var("DB_USER").unwrap_or("root".into()));
    cfg.password = Some(env::var("DB_PASSWORD").unwrap_or("1234".into()));

    let pool_size = env::var("DB_POOL_SIZE").unwrap_or("125".into()).parse::<usize>().unwrap();

    cfg.pool = PoolConfig::new(pool_size).into();
    println!("creating postgres pool...");
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
    println!("postgres pool successfully created");

    let mut cfg = deadpool_redis::Config::default();
    let redis_host = env::var("REDIS_HOST").unwrap_or("localhost".into());
    cfg.connection = Some(ConnectionInfo {
        addr: ConnectionAddr::Tcp(redis_host, 6379),
        redis: RedisConnectionInfo {
            db: 0,
            username: None,
            password: None,
        },
    });
    let redis_pool_size = env::var("REDIS_POOL_SIZE").unwrap_or("9995".into()).parse::<usize>().unwrap();
    cfg.pool = Some(PoolConfig {
        max_size: redis_pool_size,
        timeouts: Timeouts {
            wait: Some(Duration::from_secs(60)),
            create: Some(Duration::from_secs(60)),
            recycle: Some(Duration::from_secs(60)),
        }
    });
    println!("creating redis pool...");
    let redis_pool = cfg.create_pool(Some(Runtime::Tokio1))?;
    println!("redis pool successfully created");

    tokio::spawn(async move {db_warmup().await});

    let pool_async = pool.clone();
    tokio::spawn(async move { db_clean_warmup(pool_async).await });

    let pool_async = pool.clone();
    let queue = Arc::new(AppQueue::new());
    let queue_async = queue.clone();
    tokio::spawn(async move { db_flush_queue(pool_async, queue_async).await });
    
    Ok(())
}
