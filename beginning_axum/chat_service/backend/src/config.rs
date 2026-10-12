use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    #[serde(default = "default_port")]
    pub server_port: u16,
}

fn default_port() -> u16 {
    3000
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        envy::from_env::<Config>().expect("Required env vars are not set")
    }
}
