use serde::Deserialize;

use crate::database::models::page::Page;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GetPageResponse {
    pub results: Vec<Page>,
}
