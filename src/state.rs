use crate::config::Config;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: std::sync::Arc<Config>,
}

impl AppState {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let db = SqlitePool::connect(&config.database_url).await?;
        sqlx::migrate!("./migrations").run(&db).await?;
        Ok(Self {
            db,
            config: std::sync::Arc::new(config),
        })
    }
}
