mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod utils;

use std::env;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

#[tokio::main]
async fn main() {
    dotenv::from_filename(".env.development").ok();
    println!("🌟 EasyTicket API 🌟");

    let app = Router::new().route("/", get(home));

    let addr = env::var("HOST").expect("Erro ao carregar env HOST");
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            println!("✅ Servidor iniciado em {}", &addr);
            listener
        }
        Err(e) => {
            eprintln!("❌ Erro ao iniciar o servidor: {e}");
            std::process::exit(1)
        }
    };

    match db::connection::test_connection().await {
        Ok(_) => {
            println!("✅ Banco de dados iniciado");
        }
        Err(e) => {
            eprintln!("❌ Erro ao iniciar banco de dados: {e}");
            std::process::exit(1)
        }
    }

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> impl IntoResponse {
    (StatusCode::OK, "Home")
}
