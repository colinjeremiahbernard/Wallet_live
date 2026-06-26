use crate::app::App;
mod routes;
mod app;
pub mod models;
pub mod auth;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await?;
    Ok(())
}
