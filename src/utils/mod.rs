
use chrono::prelude::*;

pub fn generate_datetime_str() -> String {
    // let now = Utc::now();
    // now.to_string()
    Utc::now().to_string()
}