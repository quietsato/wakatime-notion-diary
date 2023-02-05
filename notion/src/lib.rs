mod api;
mod database;
mod page;

pub use api::NotionApi;
pub use database::get_page::{
    error::GetPageError,
    response::{GetPageResponse, Page},
    GetPage,
};
pub use page::append_blocks::{AppendBlocks, AppendBlocksError};
