use crate::error::ServerError;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config = Config::try_from_env().unwrap();
    std::env::set_var("OPENAI_API_KEY", &config.openai.api_key);
    config
});

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub log: LogConfig,
    pub ip: String,
    pub http_port: u16,
    pub server_name: String,
    pub server_type: ServerType,
    pub rate_limit: RateLimitConfig,
    pub handler: HandlerConfig,
    pub reply: ReplyConfig,
    pub openai: OpenAIConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RateLimitConfig {
    pub capacity: u64,
    pub quantum: u64,
    pub rate: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIConfig {
    pub open: bool,
    pub stream: bool,
    pub api_key: String,
    pub api_domain: String,
    pub max_tokens: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ServerType {
    KPBackend,
    CQBackend,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HandlerConfig {
    pub kp: KPHandlerConfig,
    pub cq: CQHandlerConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KPHandlerConfig {
    pub ip: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CQHandlerConfig {
    pub ip: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReplyConfig {
    pub prefix: String,
    pub cache_per_question: usize,
    pub blacklist: Vec<String>,
    pub replace: HashMap<String, Vec<String>>,
    pub text: HashMap<String, Vec<String>>,
    pub static_picture: HashMap<String, Vec<String>>,
    pub gif_picture: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogConfig {
    pub level: String,
    pub with_thread_ids: bool,
    pub with_thread_names: bool,
}

impl Config {
    pub fn try_from_env() -> Result<Config, ServerError> {
        Ok(config::Config::builder()
            .add_source(config::File::with_name("./Config.toml"))
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize::<Config>()?)
    }
}
