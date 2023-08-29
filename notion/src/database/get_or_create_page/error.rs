use std::fmt::Display;

use crate::{database::create_page::error::CreatePageError, GetPageError};

#[derive(Debug, Clone)]
pub enum GetOrCreatePageError {
    GetPageError(GetPageError),
    CreatePageError(CreatePageError),
}

impl Display for GetOrCreatePageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<GetPageError> for GetOrCreatePageError {
    fn from(value: GetPageError) -> Self {
        Self::GetPageError(value)
    }
}

impl From<CreatePageError> for GetOrCreatePageError {
    fn from(value: CreatePageError) -> Self {
        Self::CreatePageError(value)
    }
}
