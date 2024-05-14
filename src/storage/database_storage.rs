/*
 * Modular Database Storage/API Storage
 */

use std::collections::HashMap;
use std::io::Result;
use std::fs;
use crate::storage::user::User;


pub fn save_to_json(enrollment: &User) -> Result<()> {
    let mut enrollments: HashMap<u64, User> = HashMap::new();

    // Load existing data
    if let Ok(data) = fs::read_to_string("enrollments.json") {
        enrollments = serde_json::from_str(&data)?;
    }

    // Add new enrollment
    enrollments.insert(enrollment.user_id, enrollment.clone());

    // Save to file
    let data = serde_json::to_string(&enrollments)?;
    fs::write("enrollments.json", data)?;

    Ok(())
}// end save_to_json