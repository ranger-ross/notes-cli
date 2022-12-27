use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Note {
    pub title: String,
    pub body: String,
    pub create_timestamp: DateTime<Utc>,
    pub last_updated_timestamp: DateTime<Utc>,
}