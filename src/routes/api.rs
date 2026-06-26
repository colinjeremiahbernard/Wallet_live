use axum::{
    extract::State,
    routing::{get, put},
    Json,
    Router,
};

use serde::Deserialize;
use crate::{
    auth::admin::Admin,
    app::AppState,
    models::Asset,
};

pub fn router() -> Router<AppState>{
    Router::new()
        .route("/assets", get(list_assets).post(create_asset))
        .route("/assets/update", put(update_asset))
}

#[tracing::instrument(skip_all)]
async fn list_assets(
    State(state): State<AppState>,
) -> Json<Vec<Asset>> {
    let assets = state.assets.lock().await;
    Json(assets.values().cloned().collect::<Vec<_>>())
}

#[derive(Deserialize)]
struct CreateAssetRequest {
    name: String,
    unit_value: f64,
}

#[tracing::instrument(skip_all)]
#[axum::debug_handler]
async fn create_asset(
    _admin: Admin,
    State(state): State<AppState>,
    Json(request): Json<CreateAssetRequest>,
) -> Json<Asset> {
    let mut assets = state.assets.lock().await;

    let id = assets.iter().map(|(_, a)| a.id).max().unwrap_or(0) + 1;

    let new_asset = Asset {
        id,
        name: request.name,
        unit_value: request.unit_value,
    };

    assets.insert(id, new_asset.clone());

    Json(new_asset)
}

#[derive(Deserialize)]
struct UpdateAssetRequest {
    id: i64,
    name: Option<String>,
    unit_value: Option<f64>,
}

#[tracing::instrument(skip_all)]
#[axum::debug_handler]
async fn update_asset(
    _admin: Admin,
    State(state): State<AppState>,
    Json(request): Json<UpdateAssetRequest>,
) -> Result<Json<Asset>, &'static str> {
    let mut assets = state.assets.lock().await;

    let existing = assets
        .get_mut(&request.id)
        .ok_or("Asset does not exist")?;

    if let Some(new_name) = request.name {
        existing.name = new_name;
    }

    if let Some(new_value) = request.unit_value {
        existing.unit_value = new_value;
    }

    Ok(Json(existing.clone()))
}