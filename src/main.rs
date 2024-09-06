mod db;
mod handlers;
mod models;

use axum::{routing::get, Router};
use handlers::{status, ticket};
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::from_filename(".env.development").ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&database_url).await.unwrap();
    println!("🌟 EasyTicket API 🌟");

    let app = Router::new()
        .route("/status", get(status::show_status))
        .route(
            "/tickets",
            get(ticket::show_tickets).post(ticket::create_ticket),
        )
        .route("/tickets/count", get(ticket::count_tickets))
        .with_state(pool);

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
