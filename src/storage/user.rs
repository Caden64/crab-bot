use poise;
use serde::{Deserialize, Serialize};

// User type
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub user_id: u64,
    pub user_name: String,
    pub name: String,
    pub role: String,
    pub email: String,
    pub interests: String,
    pub email_distro: bool,
    pub points: i64,
    pub thm_username: String,
}

#[derive(Debug, poise::Modal)]
#[allow(dead_code)] // fields only used for Debug print
pub struct EditableUser {
    #[min_length = 1]
    #[max_length = 50]
    pub name: Option<String>,
    #[min_length = 2]
    #[max_length = 50]
    pub email: Option<String>,
    #[min_length = 1]
    #[max_length = 50]
    pub thm_username: Option<String>,
}
