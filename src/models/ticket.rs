use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub enum TicketError {
    EmptyTitle,
    TitleTooShort,
    TitleTooBig,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TicketStatus {
    Open,
    Pending,
    Completed,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ticket {
    pub id: Uuid,
    pub title: String,
    pub requester: String,
    pub created_at: DateTime<Utc>,
    pub status: TicketStatus,
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
            status: TicketStatus::Open,
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewTicket {
    pub title: String,
    pub requester: String,
}

impl NewTicket {
    pub fn new(title: &str, requester: &str) -> Self {
        NewTicket {
            title: String::from(title),
            requester: String::from(requester),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeleteTicket {
    pub id: Uuid,
}
