mod error;
mod response;

use self::response::Page;
pub use self::{error::GetPageError, response::GetPageResponse};
use crate::api::NotionApi;
use serde::Serialize;

pub trait GetPage {
    fn get_page(&self, query: impl Serialize) -> Result<Page, GetPageError>;
}

impl GetPage for NotionApi {
    fn get_page(&self, query: impl Serialize) -> Result<Page, GetPageError> {
        let res = reqwest::blocking::Client::new()
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
            .map_err(GetPageError::http_error)?
            .json::<GetPageResponse>()
            .unwrap();

        res.results
            .first()
            .ok_or(GetPageError::NotFound)
            .map(ToOwned::to_owned)
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use uuid::uuid;

    use crate::api::NotionApi;

    use super::GetPage;

    #[ignore]
    #[test]
    fn test_get_page() {
        let api = NotionApi::new("", &uuid!("b2f8f49f-9ed0-4c04-a282-38b4adc504ad"));

        let res = api.get_page(json!({
            "filter": {
                "and": [
                    {
                        "property": "Date",
                        "date": {
                            "on_or_after": "2023-02-04T05:54:36Z"
                        }
                    },
                    {
                        "property": "Date",
                        "date": {
                            "before": "2023-02-05T05:54:36Z"
                        }
                    },
                ]
            }
        }));

        assert!(res.is_ok(), "{:?}", res.unwrap_err());
        dbg!(res.unwrap());
    }
}
