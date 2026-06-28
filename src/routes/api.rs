use axum::{
    extract::{ Json, Path },
    routing::{get, put},
    Router,
};

use serde::Deserialize;

use crate::{
    app::AppState,
    auth::admin::Admin,
    error::AppError,
    models::Asset,
    repository::Repository,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/assets", get(list_assets).post(create_asset))
        .route("/assets/update", put(update_asset))
}

#[tracing::instrument(skip_all)]
async fn list_assets(
    repository: Repository,
) -> Result<Json<Vec<Asset>>, AppError> {
    let assets = repository.list_assets().await?;

    Ok(Json(assets))
}

#[derive(Deserialize)]
struct CreateAssetRequest {
    name: String,
    unit_value: f64,
}

#[tracing::instrument(skip_all)]
#[axum::debug_handler(state = AppState)]
async fn create_asset(
    _admin: Admin,
    repository:Repository,
    Json(request): Json<CreateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    let asset = repository
        .create_asset(request.name, request.unit_value)
        .await?;

    Ok(Json(asset))
}

#[derive(Deserialize)]
struct UpdateAssetRequest {
    id: i32,
    name: Option<String>,
    unit_value: Option<f64>,
}

#[tracing::instrument(skip_all)]
#[axum::debug_handler(state = AppState)]
async fn update_asset(
    _admin: Admin,
    repository:Repository,
    Json(request): Json<UpdateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    let asset = repository
        .update_asset(
            request.id,
            request.name,
            request.unit_value,
        )
        .await?
        .ok_or(AppError::AssetDoesNotExist)?;

    Ok(Json(asset))
}