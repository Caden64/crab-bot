use crate::storage::user::User;
use std::collections::HashMap;
use std::fs;

pub fn user_exists(user_id: &u64) -> bool {
    let mut enrollments: HashMap<u64, User> = HashMap::new();

    // Load existing data
    
    if let Ok(data) = fs::read_to_string("enrollments.json") {
        let user_enrollments_result = serde_json::from_str(&data);
        if let Ok(user_enrollments_result) = user_enrollments_result {
            enrollments = user_enrollments_result
        } else {
            println!("Unable to read user registration file\n Error: {}", user_enrollments_result.expect_err(""));
            return false;
        }
    }

    enrollments.contains_key(user_id)
}
