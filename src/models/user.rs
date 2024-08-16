use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    id: u32,
    pub username: String,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}
