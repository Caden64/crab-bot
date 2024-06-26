use crate::storage::user::User;
use std::collections::HashMap;
use std::fs;

pub fn get_user(user_id: &u64) -> Option<User> {
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
        return Some(user.clone());
    }

    println!("User did not exist");
    None
}
