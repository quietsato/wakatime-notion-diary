use serde::Deserialize;

use super::grand_total::GrandTotal;
use super::language::Language;
use super::projects::Project;

#[derive(Debug, Deserialize, PartialEq)]
pub struct SummariesData {
    pub grand_total: GrandTotal,
    pub languages: Vec<Language>,
    pub projects: Vec<Project>,
}
