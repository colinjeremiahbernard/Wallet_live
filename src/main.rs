use crate::app::App;

mod app;
mod routes;

pub mod auth;
pub mod error;
pub mod models;
pub mod repository;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await?;
    Ok(())
}