use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::AppState;
use crate::db_models::User;


pub async fn index() -> &'static str {
    "index route"
}

pub async fn user(
    State(state): State<Arc<AppState>>
) -> Result<Json<User>, StatusCode> {    
    let pulled_user = sqlx::query_as!(User, "SELECT * FROM users").fetch_one(&state.pool).await;
    if let Ok(user) = pulled_user {
        Ok(Json(user))
    }
    else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}
