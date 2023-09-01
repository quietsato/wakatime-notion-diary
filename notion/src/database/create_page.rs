pub mod error;
pub mod response;

use self::error::CreatePageError;
use crate::{CreatePageResponse, NotionApi, Page};
use serde::Serialize;
use serde_json::json;
use std::io::Read;

pub trait CreatePage {
    fn create_page(&self, query: impl Serialize) -> Result<Page, CreatePageError>;
}

impl CreatePage for NotionApi {
    fn create_page(&self, query: impl Serialize) -> Result<Page, CreatePageError> {
        let query = {
            let mut query = json!(query);
            query
                .as_object_mut()
                .unwrap()
                .insert("parent".into(), json!({"database_id": self.database_id}));
            query
        };
        let buf = {
            let mut res = reqwest::blocking::Client::new()
                .request(reqwest::Method::POST, "https://api.notion.com/v1/pages")
                .bearer_auth(&self.api_key)
                .header("Notion-Version", "2022-06-28")
                .json(&query)
                .send()
                .map_err(CreatePageError::http_error)?;

            let mut buf = String::new();
            res.read_to_string(&mut buf)
                .map_err(CreatePageError::parse_error)?;
            buf
        };
        serde_json::from_str::<CreatePageResponse>(&buf)
            .map_err(|e| CreatePageError::parse_error(format!("{e}\n{buf}")))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
    use std::str::FromStr;
    use uuid::Uuid;

    #[ignore]
    #[test]
    fn test_create_page() {
        dotenv::dotenv().ok();

        let api = NotionApi::new(
            std::env::var("NOTION_API_KEY").unwrap(),
            &Uuid::from_str(&std::env::var("NOTION_DATABASE_ID").unwrap()).unwrap(),
        );

        let res = api.create_page(json!({
            "properties": {
                "Name": {
                    "title": [
                        {
                            "text": {
                                "content": "Test"
                            }
                        }
                    ]
                },
            },
        }));

        assert!(res.is_ok(), "{:?}", res.unwrap_err());
        dbg!(res.unwrap());
    }
}
