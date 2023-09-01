pub mod error;
pub mod response;

use self::error::GetPageError;
use crate::{GetPageResponse, NotionApi, Page};
use serde::Serialize;
use std::io::Read;

pub trait GetPage {
    fn get_page(&self, query: impl Serialize) -> Result<Page, GetPageError>;
}

impl GetPage for NotionApi {
    fn get_page(&self, query: impl Serialize) -> Result<Page, GetPageError> {
        let mut res = reqwest::blocking::Client::new()
            .request(
                reqwest::Method::POST,
                format!(
                    "https://api.notion.com/v1/databases/{}/query",
                    &self.database_id
                ),
            )
            .bearer_auth(&self.api_key)
            .header("Notion-Version", "2022-06-28")
            .json(&query)
            .send()
            .map_err(GetPageError::http_error)?;

        let res = {
            let mut buf = String::new();
            res.read_to_string(&mut buf)
                .map_err(GetPageError::parse_error)?;
            serde_json::from_str::<GetPageResponse>(&buf)
                .map_err(|e| GetPageError::parse_error(format!("{e}\n{buf}")))?
        };

        res.results
            .first()
            .ok_or(GetPageError::NotFound)
            .map(ToOwned::to_owned)
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
    fn test_get_page() {
        dotenv::dotenv().ok();

        let api = NotionApi::new(
            std::env::var("NOTION_API_KEY").unwrap(),
            &Uuid::from_str(&std::env::var("NOTION_DATABASE_ID").unwrap()).unwrap(),
        );

        let res = api.get_page(json!({
            "filter": {
                "and": [
                    {
                        "property": "Date",
                        "date": {
                            "on_or_after": "2023-06-01T00:00:00Z"
                        }
                    },
                    {
                        "property": "Date",
                        "date": {
                            "before": "2023-06-02T00:00:00Z"
                        }
                    },
                ]
            }
        }));

        assert!(res.is_ok(), "{:?}", res.unwrap_err());
        dbg!(res.unwrap());
    }
}
