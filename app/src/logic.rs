use chrono::{DateTime, Days, TimeZone, Timelike, Utc};
use serde::Serialize;
use serde_json::json;
use wakatime_notion_diary_wakatime::summaries::Summaries;

pub trait Logic {
    type Query: Serialize;
    type Blocks: Serialize;

    fn build_database_query(&self) -> Result<Self::Query, String>;
    fn build_notion_page_blocks(
        &self,
        wakatime_summaries: &Summaries,
    ) -> Result<Self::Blocks, String>;
}

#[derive(Debug)]
pub struct AppLogic<Tz: TimeZone> {
    today: DateTime<Tz>,
}

impl<Tz: TimeZone> AppLogic<Tz> {
    pub fn new(today: DateTime<Tz>) -> Self {
        Self { today }
    }
}

impl<Tz: TimeZone> Logic for AppLogic<Tz> {
    type Query = serde_json::Value;
    type Blocks = serde_json::Value;

    fn build_database_query(&self) -> Result<Self::Query, String> {
        let day_start = self
            .today
            .with_hour(0)
            .and_then(|day| day.with_minute(0))
            .and_then(|day| day.with_second(0))
            .and_then(|day| day.with_nanosecond(0))
            .ok_or("Failed to construct day_start")?
            .with_timezone(&Utc);
        let day_end = day_start
            .checked_add_days(Days::new(1))
            .ok_or("Failed to construct day_end")?;

        Ok(json!({
            "filter": {
                "and": [
                    {
                        "property": "Date",
                        "date": {
                            "on_or_after": day_start.to_rfc3339(),
                        }
                    },
                    {
                        "property": "Date",
                        "date": {
                            "before": day_end.to_rfc3339(),
                        }
                    },
                ]
            }
        }))
    }

    fn build_notion_page_blocks(
        &self,
        wakatime_summaries: &Summaries,
    ) -> Result<Self::Blocks, String> {
        let data = &wakatime_summaries.data.first().unwrap();

        let mut blocks = vec![json!({
            "object": "block",
            "type": "heading_2",
            "heading_2": {
                "rich_text": [{ "type": "text", "text": { "content": "Coding Activities" } }]
            }
        })];

        blocks.push(json!({
            "object": "block",
            "type": "paragraph",
            "paragraph": {
                "rich_text": [
                    {
                        "type": "text",
                        "text": { "content": format!("Total: {}", data.grand_total.text) }
                    }
                ]
            }
        }));

        for language in data.languages.iter() {
            blocks.push(json!({
                "object": "block",
                "type": "numbered_list_item",
                "numbered_list_item": {
                    "rich_text": [
                        {
                            "type": "text",
                            "text": { "content": format!("{} {} ({:.1}%)", language.name, language.text, language.percent) }
                        }
                    ],
                    "color": "default",
                    "children": [],
                }
            }))
        }

        Ok(json!({ "children": blocks }))
    }
}
