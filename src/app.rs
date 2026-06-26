use std::sync::Arc;
use tokio::sync::Mutex;
use axum::Router;
use crate::models::Asset;
use tokio::net::TcpListener;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::Layer;
use tracing_subscriber::util::SubscriberInitExt;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use std::collections::HashMap;
use crate::routes::api;

#[derive(Clone)]
pub struct AppState {
 pub assets: Arc<Mutex<HashMap<i64, Asset>>>,
}
impl AppState {
  fn new() -> Self {
    Self{
      assets: Default::default(),
    }
  }
}
pub struct App;

impl App {
  pub async fn start() -> color_eyre::Result<()> {
      let layer = tracing_subscriber::fmt::layer()
    .with_span_events(FmtSpan::NEW)
    .boxed();
    let _ = tracing_subscriber::registry().with(layer).try_init();

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await?;

    let router = Router::new()
        .nest("/api", api::router())
        .with_state(AppState::new());
    
    info!("Starting service");

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, router)
        .await?;
        Ok(())
  }
}
