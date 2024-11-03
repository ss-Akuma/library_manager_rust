use rand::{distributions::Alphanumeric, Rng};
use std::time::{SystemTime, UNIX_EPOCH};


pub fn generate_custom_id(length: usize) -> String {
    let random_part: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length - 5)  // Adjust based on the desired length
        .map(char::from)
        .collect();
    
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    format!("{}{}", random_part, timestamp)
}



