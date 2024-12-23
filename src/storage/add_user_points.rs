use crate::storage::save_user::save_to_json;
use crate::storage::user::User;
use std::collections::HashMap;
use std::fs;

pub fn add_user_points(user_id: &u64, points: u16) -> Option<()> {
    let mut enrollments: HashMap<u64, User> = HashMap::new();

    // Load existing data
    if let Ok(data) = fs::read_to_string("enrollments.json") {
        let user_enrollments_result = serde_json::from_str(&data);
        if let Ok(user_enrollments_result) = user_enrollments_result {
            enrollments = user_enrollments_result
        } else {
            println!("Unable to read user registration file");
            return None;
        }
    }

    // edit existing data
    if let Some(user) = enrollments.get(user_id) {
        let mut user = user.clone();
        user.points += points as i64;
        if save_to_json(&user).is_err() {
            println!("Unable to save user registration file");
            return None;
        }
    }

    Some(())
}
