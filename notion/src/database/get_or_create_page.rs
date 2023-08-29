pub mod error;
pub mod response;

use self::error::GetOrCreatePageError;
use super::{create_page::CreatePage, models::page::Page};
use crate::{GetPage, GetPageError};
use serde::Serialize;

pub trait GetOrCreatePage {
    fn get_or_create_page(
        &self,
        get_query: impl Serialize,
        create_query: impl Serialize,
    ) -> Result<Page, GetOrCreatePageError>;
}

impl<T> GetOrCreatePage for T
where
    T: GetPage + CreatePage,
{
    fn get_or_create_page(
        &self,
        get_query: impl Serialize,
        create_query: impl Serialize,
    ) -> Result<Page, GetOrCreatePageError> {
        match self.get_page(get_query) {
            Ok(page) => Ok(page),
            Err(GetPageError::NotFound) => self.create_page(create_query).map_err(Into::into),
            Err(e) => Err(e.into()),
        }
    }
}
