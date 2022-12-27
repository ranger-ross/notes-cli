use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub title: String,
    pub body: String,
    pub create_timestamp: i64,
    pub last_updated_timestamp: i64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseSchema {
    pub config: AppConfig,
    pub notes: Vec<Note>
}