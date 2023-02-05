use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Language {
    pub name: String,
    pub total_seconds: f32,
    pub percent: f32,
    pub digital: String,
    pub text: String,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}
