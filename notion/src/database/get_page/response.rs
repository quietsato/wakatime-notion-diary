use serde::Deserialize;

use crate::Page;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GetPageResponse {
    pub results: Vec<Page>,
}
