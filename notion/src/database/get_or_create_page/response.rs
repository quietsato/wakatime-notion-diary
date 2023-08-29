use serde::Deserialize;

use crate::database::models::page::Page;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GetOrCreatePageResponse {
    pub result: Vec<Page>,
}
