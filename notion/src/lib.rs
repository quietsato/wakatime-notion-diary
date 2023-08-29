mod api;
mod database;
mod page;

pub use api::NotionApi;
pub use database::create_page::{error::CreatePageError, response::CreatePageResponse, CreatePage};
pub use database::get_or_create_page::{
    error::GetOrCreatePageError, response::GetOrCreatePageResponse, GetOrCreatePage,
};
pub use database::get_page::{error::GetPageError, response::GetPageResponse, GetPage};
pub use database::models::page::Page;
pub use page::append_blocks::{AppendBlocks, AppendBlocksError};
