use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,sqlx::FromRow)]
pub struct Ticket{
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct CreateTicket{
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct UpdateTicket{
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}