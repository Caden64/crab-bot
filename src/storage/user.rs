use serde::{Deserialize, Serialize};

// User type
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub user_id: u64,
    pub user_name: String,
    pub name: String,
    pub role: String,
    pub email: String,
    pub points: i64,
}
