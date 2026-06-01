#[derive(serde::Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_database_url")]
    pub database_url: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_jwt_secret")]
    pub jwt_secret: String,
}

fn default_database_url() -> String {
    "sqlite://app.db".into()
}
fn default_port() -> u16 {
    3000
}
fn default_jwt_secret() -> String {
    "secret".into()
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        envy::from_env::<Config>().map_err(Into::into)
    }
}
