use std::sync::Arc;

use sqlx::PgPool;
use axum::{routing::get, Router};

mod routes;
mod db_models;


#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!().run(&pool).await.expect("Migrations failed");

    let state = Arc::new(AppState { pool });
    let router = Router::new()
        .route("/", get(routes::index))
        .route("/user", get(routes::user))
        .with_state(state);

    Ok(router.into())
}
