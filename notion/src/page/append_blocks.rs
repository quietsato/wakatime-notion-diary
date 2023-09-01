use std::fmt::Display;

use serde::Serialize;
use uuid::Uuid;

use crate::api::NotionApi;

#[derive(Debug, Serialize)]
pub enum AppendBlocksError {
    HttpError(String),
}

impl Display for AppendBlocksError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl AppendBlocksError {
    pub fn http_error(err: impl ToString) -> Self {
        Self::HttpError(err.to_string())
    }
}

pub trait AppendBlocks {
    fn append_blocks(
        &self,
        page_id: &Uuid,
        blocks: impl Serialize,
    ) -> Result<(), AppendBlocksError>;
}

impl AppendBlocks for NotionApi {
    fn append_blocks(
        &self,
        page_id: &Uuid,
        blocks: impl Serialize,
    ) -> Result<(), AppendBlocksError> {
        reqwest::blocking::Client::new()
            .request(
                reqwest::Method::PATCH,
                format!("https://api.notion.com/v1/blocks/{page_id}/children"),
            )
            .bearer_auth(&self.api_key)
            .header("Notion-Version", "2022-06-28")
            .json(&blocks)
            .send()
            .map(|_| ())
            .map_err(AppendBlocksError::http_error)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::NotionApi;
    use serde_json::json;
    use std::str::FromStr;

    #[ignore]
    #[test]
    fn test_append_blocks() {
        let api = NotionApi::new("api_key", &Uuid::from_str("page_id").unwrap());

        let res = api.append_blocks(
            &Uuid::from_str("page_id").unwrap(),
            json!({
                "children": [
                    {
                        "object": "block",
                        "type": "heading_2",
                        "heading_2": {
                            "rich_text": [{ "type": "text", "text": { "content": "Lacinato kale" } }]
                        }
                    },
                    {
                        "object": "block",
                        "type": "paragraph",
                        "paragraph": {
                            "rich_text": [
                                {
                                    "type": "text",
                                    "text": {
                                        "content": "Lacinato kale is a variety of kale with a long tradition in Italian cuisine, especially that of Tuscany. It is also known as Tuscan kale, Italian kale, dinosaur kale, kale, flat back kale, palm tree kale, or black Tuscan palm.",
                                        "link": { "url": "https://en.wikipedia.org/wiki/Lacinato_kale" }
                                    }
                                }
                            ]
                        }
                    }
                ]
            }),
        );

        assert!(res.is_ok(), "{:?}", res.unwrap_err());
    }
}
