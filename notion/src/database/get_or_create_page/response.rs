use serde::Deserialize;

use crate::Page;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GetOrCreatePageResponse {
    pub result: Vec<Page>,
}
