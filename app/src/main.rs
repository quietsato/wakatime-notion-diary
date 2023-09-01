mod logic;

use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use logic::{AppLogic, Logic};
use uuid::Uuid;
use wakatime_notion_diary_notion::{AppendBlocks, GetOrCreatePage, NotionApi};
use wakatime_notion_diary_wakatime::summaries::SummariesRequest;

pub struct AppConfig<Tz: TimeZone> {
    pub today: DateTime<Tz>,
    pub wakatime_api_key: String,
    pub notion_api_key: String,
    pub notion_database_id: Uuid,
}

pub fn main() {
    dotenv::dotenv().ok();

    let now = if let Some(arg_datetime) = std::env::args().nth(1) {
        // Parse command line argument
        DateTime::parse_from_rfc3339(&arg_datetime)
            .expect(&format!("Failed to parse rfc3339: {arg_datetime}"))
    } else {
        // Get current time
        let jst = FixedOffset::east_opt(9 * 3600).unwrap();
        Utc::now().with_timezone(&jst)
    };
    println!("Current Time: {}", now.to_rfc3339());

    let config = AppConfig {
        today: now,
        wakatime_api_key: std::env::var("WAKATIME_API_KEY").unwrap(),
        notion_api_key: std::env::var("NOTION_API_KEY").unwrap(),
        notion_database_id: Uuid::parse_str(&std::env::var("NOTION_DATABASE_ID").unwrap()).unwrap(),
    };
    let logic = AppLogic::new(config.today);

    println!("Fetching Wakatime Status");
    let wakatime_summaries =
        match SummariesRequest::new(&config.wakatime_api_key, config.today).get() {
            Ok(res) => res,
            Err(err) => panic!("Failed to get wakatime summaries: {err}"),
        };

    // Build queries
    println!("Building Notion Queries");
    let (get_query, create_query) = {
        let get_query = logic
            .build_get_page_query()
            .expect("Failed to build get query for database");
        let create_query = logic
            .build_create_page_query()
            .expect("Failed to build create query for database");
        (get_query, create_query)
    };

    println!("Fetching or Creating Notion Page");
    let notion_api = NotionApi::new(&config.notion_api_key, &config.notion_database_id);
    let page = match notion_api.get_or_create_page(get_query, create_query) {
        Ok(page) => page,
        Err(err) => panic!("Failed to get notion page: {err}"),
    };

    println!("Building Notion Page Blocks");
    let blocks = match logic.build_notion_page_blocks(&wakatime_summaries) {
        Ok(blocks) => blocks,
        Err(err) => panic!("Failed to build notion page blocks: {err}"),
    };

    println!("Appending Notion Page Blocks");
    match notion_api.append_blocks(&page.id, blocks) {
        Ok(_) => (),
        Err(err) => panic!("Failed to append blocks to notion page: {err}"),
    };

    println!("All Done!");
}
