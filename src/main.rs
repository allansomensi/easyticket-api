mod handlers;
mod models;

use axum::{routing::get, Router};
use handlers::{status, ticket};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();

    println!("🌟 EasyTicket API 🌟");

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Conectado ao banco de dados");
            pool
        }
        Err(e) => {
            eprintln!("❌ Erro ao se conectar ao banco de dados: {e}");
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/status", get(status::show_status))
        .route(
            "/tickets",
            get(ticket::show_tickets)
                .post(ticket::create_ticket)
                .delete(ticket::delete_ticket),
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

    axum::serve(listener, app).await.unwrap();
}
