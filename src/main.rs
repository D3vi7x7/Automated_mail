use axum::{Router,routing::get};
use sqlx::any;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use dotenvy::dotenv;
use std::env;
use tracing_subscriber;

mod config;
mod db;
mod auth;
mod models;
mod routes;
mod emails;

#[tokio::main]

async fn main(){
    dotenv().ok();

    let pool = db::connect_pg().await.expect("DB CONNECTION FAILED !!!");

    emails::email_listener::start_email_watcher(pool.clone()).await;

    let app = Router::new()
    .merge(routes::create_auth_routes(pool.clone()))
    .merge(routes::create_ticket_routes(pool.clone()))
    .route("/", get(|| async {"Ticketing system backend running !!"}))
    .layer(CorsLayer::new().allow_origin(Any));

    let port = env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse::<u16>()
    .unwrap();

    let addr = SocketAddr::from(([0,0,0,0],port));

    println!("Server running on port {}",addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app.into_make_service()).await.unwrap();
}