#[warn(clippy::pedantic)]
pub mod app;
mod character;
mod connection;
mod quic;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;
