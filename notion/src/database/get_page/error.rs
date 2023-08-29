use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum GetPageError {
    HttpError(String),
    ParseError(String),
    NotFound,
}

impl Display for GetPageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl GetPageError {
    pub fn http_error(err: impl ToString) -> Self {
        Self::HttpError(err.to_string())
    }

    pub fn parse_error(err: impl ToString) -> Self {
        Self::ParseError(err.to_string())
    }
}
