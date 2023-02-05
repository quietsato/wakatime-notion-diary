use uuid::Uuid;

#[derive(Debug)]
pub struct NotionApi {
    pub(crate) api_key: String,
    pub(crate) database_id: String,
}

impl NotionApi {
    pub fn new(api_key: impl ToString, database_id: &Uuid) -> Self {
        Self {
            api_key: api_key.to_string(),
            database_id: database_id.to_string(),
        }
    }
}
