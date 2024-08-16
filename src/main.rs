mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod utils;

use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv::from_filename(".env.development").ok();
    let app = Router::new().route("/", get("Hello, world!"));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("=> Listening on http://localhost:8080...");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
