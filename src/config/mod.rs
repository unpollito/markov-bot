use dotenv::dotenv;
use std::env;

pub struct Config {
    pub bot_token: String,
    pub mongodb_url: String,
    pub mongodb_user: String,
    pub mongodb_password: String,
    pub mongodb_db: String,
    pub log_level: Option<String>,
}

pub fn load_config_from_env() -> Config {
    if let Err(e) = dotenv() {
        panic!("Failed to read env file: {}", e);
    }
    let mut config = Config {
        bot_token: String::from(""),
        mongodb_url: String::from(""),
        mongodb_user: String::from(""),
        mongodb_password: String::from(""),
        mongodb_db: String::from(""),
        log_level: None,
    };
    for (key, value) in env::vars() {
        match key.as_str() {
            "TELEGRAM_BOT_TOKEN" => config.bot_token = String::from(value),
            "MONGODB_URL" => config.mongodb_url = String::from(value),
            "MONGODB_USER" => config.mongodb_user = String::from(value),
            "MONGODB_PASSWORD" => config.mongodb_password = String::from(value),
            "MONGODB_DB" => config.mongodb_db = String::from(value),
            "RUST_LOG" => config.log_level = Some(String::from(value)),
            _ => (),
        }
    }

    if config.bot_token.is_empty() {
        panic!("TELEGRAM_BOT_TOKEN not set");
    } else if config.mongodb_url.is_empty() {
        panic!("MONGODB_URL not set");
    } else if config.mongodb_user.is_empty() {
        panic!("MONGODB_USER not set");
    } else if config.mongodb_password.is_empty() {
        panic!("MONGODB_PASSWORD not set");
    } else if config.mongodb_db.is_empty() {
        panic!("MONGODB_DB not set");
    }

    config
}
