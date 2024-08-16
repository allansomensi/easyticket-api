use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Deserialize, Serialize)]
enum TicketStatus {
    Open,
    Pending,
    Concluded,
}

#[derive(Deserialize, Serialize)]
struct Ticket {
    pub id: u32,
    title: String,
    description: Option<String>,
    requester: User,
    status: TicketStatus,
    created_at: DateTime<Utc>,
    closed_at: DateTime<Utc>,
    closed_by: User,
}
