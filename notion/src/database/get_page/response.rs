use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GetPageResponse {
    pub results: Vec<Page>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Page {
    pub archived: bool,
    pub cover: serde_json::Value,
    pub created_by: serde_json::Value,
    pub created_time: chrono::DateTime<Utc>,
    pub id: Uuid,
    pub last_edited_by: serde_json::Value,
    pub last_edited_time: chrono::DateTime<Utc>,
    pub object: String,
    pub parent: serde_json::Value,
    pub properties: serde_json::Value,
    pub url: String,
}
