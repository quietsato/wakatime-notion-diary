use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct GrandTotal {
    pub digital: String,
    pub hours: u32,
    pub minutes: u32,
    pub text: String,
    pub total_seconds: f32,
}
