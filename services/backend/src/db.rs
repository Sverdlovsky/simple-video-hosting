use anyhow::Context;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;
use std::fs::File;
use serde::Deserialize;
use serde_json;
use num_cpus;

#[derive(Debug, Deserialize)]
struct FileCreds {
    user: String,
    pass: String,
    host: String,
    port: u16,
    db: String,
}

pub async fn init_db() -> anyhow::Result<Pool<Postgres>> {
    let file = File::open("keys/postgres.json").context("Failed to open keys/postgres.json")?;
    let fc: FileCreds =
        serde_json::from_reader(file).context("Failed to parse keys/postgres.json")?;

    let dsn = format!(
        "postgres://{}:{}@{}:{}/{}",
        fc.user, fc.pass, fc.host, fc.port, fc.db
    );

    let pool = PgPoolOptions::new()
        .max_connections(num_cpus::get() as u32 * 2)
        .idle_timeout(Duration::from_secs(300))
        .connect(dsn.as_str())
        .await
        .context("Failed to connect to Postgres")?;

    Ok(pool)
}
