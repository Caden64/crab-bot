use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub user_id: u64,
    pub user_name: String,
    pub name: String,
    pub university: String,
    pub email: String,
    pub interests: String,
    pub email_distro: bool,
    pub points: u64,
}
