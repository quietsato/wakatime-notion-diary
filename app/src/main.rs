mod logic;
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use logic::{AppLogic, Logic};
use uuid::Uuid;
use wakatime_notion_diary_notion::{AppendBlocks, GetPage, NotionApi};
use wakatime_notion_diary_wakatime::summaries::SummariesRequest;

pub struct AppConfig<Tz: TimeZone> {
    pub today: DateTime<Tz>,
    pub wakatime_api_key: String,
    pub notion_api_key: String,
    pub notion_database_id: Uuid,
}

pub fn main() {
    dotenv::dotenv().ok();

    let jst = FixedOffset::east_opt(9 * 3600).unwrap();
    let config = AppConfig {
        today: Utc::now().with_timezone(&jst),
        wakatime_api_key: std::env::var("WAKATIME_API_KEY").unwrap(),
        notion_api_key: std::env::var("NOTION_API_KEY").unwrap(),
        notion_database_id: Uuid::parse_str(&std::env::var("NOTION_DATABASE_ID").unwrap()).unwrap(),
    };
    let logic = AppLogic::new(config.today);

    let wakatime_summaries = match SummariesRequest::new(&config.wakatime_api_key).get() {
        Ok(res) => res,
        Err(err) => panic!("Failed to get wakatime summaries: {err}"),
    };

    let query = logic
        .build_database_query()
        .expect("Failed to build query for database");
    let notion_api = NotionApi::new(&config.notion_api_key, &config.notion_database_id);
    let page = match notion_api.get_page(query) {
        Ok(page) => page,
        Err(err) => panic!("Failed to get notion page: {err}"),
    };

    let blocks = match logic.build_notion_page_blocks(&wakatime_summaries) {
        Ok(block) => block,
        Err(err) => panic!("Failed to build notion page blocks: {err}"),
    };
    match notion_api.append_blocks(&page.id, blocks) {
        Ok(_) => (),
        Err(err) => panic!("Failed to append blocks to notion page: {err}"),
    };
}
