use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("configuration failed: {0}")]
    Config(#[from] ConfigError),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to load variable: not found '{0}'")]
    LoadVariable(String),
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("failed to establish database: {0}")]
    EstablishDatabase(sqlx::Error),

    #[error("failed to execute query: {0}")]
    ExecuteQuery(sqlx::Error),
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("failed to bind listener: {0}")]
    BindListener(tokio::io::Error),

    #[error("failed to run server: {0}")]
    Run(std::io::Error),
}

