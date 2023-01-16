use std::fmt::Display;

#[derive(Debug)]
pub enum SummariesRequestError {
    HttpError(String),
    ParseError(String),
}

impl SummariesRequestError {
    pub fn http_error(err: impl ToString) -> Self {
        Self::HttpError(err.to_string())
    }
    pub fn parse_error(err: impl ToString) -> Self {
        Self::ParseError(err.to_string())
    }
}

impl Display for SummariesRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
