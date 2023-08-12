use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreateError {
    #[error("Failed to create config directory")]
    Config(#[source] std::io::Error),
    #[error("Failed to create cache directory")]
    Cache(#[source] std::io::Error),
    #[error("Failed to create data directory")]
    Data(#[source] std::io::Error),
}

#[derive(Error, Debug)]
pub enum RemoveError {
    #[error("Failed to create config directory")]
    Config(#[source] std::io::Error),
    #[error("Failed to create cache directory")]
    Cache(#[source] std::io::Error),
    #[error("Failed to create data directory")]
    Data(#[source] std::io::Error),
}
