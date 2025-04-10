use sqlx;
use serde::{Serialize, Deserialize};


#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
}
