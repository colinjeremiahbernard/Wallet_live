use std::future::Future;

use axum::{
    extract::FromRequestParts,
    http::{
        header::AUTHORIZATION,
        request::Parts,
    },
};
use crate::app::AppState;

const ADMIN_SECRET_KEY: &str = "im-the-admin";

pub struct Admin;

impl FromRequestParts<AppState> for Admin {
    type Rejection = &'static str;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let auth = parts
                .headers
                .get(AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
                .ok_or("Missing or invalid Authorization header")?;

            // Optional: support "Bearer token"
            let token = auth.strip_prefix("Bearer ").unwrap_or(auth);

            if token == ADMIN_SECRET_KEY {
                Ok(Admin)
            } else {
                Err("Invalid credentials")
            }
        }
    }
}