use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug)]
pub enum TicketError {
    EmptyTitle,
    TitleTooShort,
    TitleTooBig,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Ticket {
    pub id: Uuid,
    pub title: String,
    pub requester: String,
    pub created_at: DateTime<Utc>,
}

impl Ticket {
    pub fn new(title: &str, requester: &str) -> Result<Self, TicketError> {
        if title.is_empty() {
            return Err(TicketError::EmptyTitle);
        }

        if title.len() < 8 {
            return Err(TicketError::TitleTooShort);
        }

        if title.len() > 40 {
            return Err(TicketError::TitleTooBig);
        }

        Ok(Ticket {
            id: Uuid::now_v7(),
            title: String::from(title),
            requester: String::from(requester),
            created_at: Utc::now(),
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateTicketRequest {
    pub title: String,
    pub requester: String,
}
