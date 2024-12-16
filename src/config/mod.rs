use crate::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn get_ip_address() -> Result<String> {
    Ok(env::var("IP_ADDRESS")
        .map_err(|_| ConfigError::LoadVariable("IP_ADDRESS".to_string()))?)
}

pub fn get_database_url() -> Result<String> {
    Ok(env::var("DATABASE_URL")
        .map_err(|_| ConfigError::LoadVariable("DATABASE_URL".to_string()))?)
}

pub fn init_configuration() {
    env::set_var("RUST_LOG", "DEBUG");

    dotenv().ok();
    env_logger::init();
}

