#[warn(clippy::pedantic)]
pub mod app;
mod authentication;
mod character;
mod connection;
mod postgres;
mod quic;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;
