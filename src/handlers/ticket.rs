use crate::models::ticket::{CreateTicketRequest, DeleteTicketRequest, Ticket};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;

pub async fn show_tickets(State(pool): State<PgPool>) -> Json<Vec<Ticket>> {
    let row: Vec<Ticket> = sqlx::query_as("SELECT * FROM tickets")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(row)
}

pub async fn count_tickets(State(pool): State<PgPool>) -> Json<i32> {
    let row: (i32,) = sqlx::query_as("SELECT COUNT(*)::int FROM tickets")
        .fetch_one(&pool)
        .await
        .unwrap();
    Json(row.0)
}

pub async fn create_ticket(
    State(pool): State<PgPool>,
    Json(request): Json<CreateTicketRequest>,
) -> impl IntoResponse {
    let new_ticket = Ticket::new(&request.title, &request.requester).unwrap();

    match sqlx::query(
        "
        INSERT INTO tickets (id, title, requester, created_at)
        VALUES ($1, $2, $3, $4)
        ",
    )
    .bind(new_ticket.id)
    .bind(&new_ticket.title)
    .bind(&new_ticket.requester)
    .bind(new_ticket.created_at)
    .execute(&pool)
    .await
    {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_ticket(
    State(pool): State<PgPool>,
    Json(request): Json<DeleteTicketRequest>,
) -> impl IntoResponse {
    match sqlx::query("DELETE FROM tickets WHERE id = $1")
        .bind(request.id)
        .execute(&pool)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
