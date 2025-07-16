use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDateTime};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User{
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct RegisterPayload{
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload{
    pub email: String,
    pub password: String,
}