use chrono::prelude::*;
use uuid::Uuid;

pub fn generate_datetime_string() -> String {
    Utc::now().to_string()
}

pub fn generate_uuid_string() -> String {
    Uuid::new_v4().to_string()
}