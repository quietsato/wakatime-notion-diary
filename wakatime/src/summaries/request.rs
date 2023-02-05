use super::{Summaries, SummariesRequestError};
use base64::{self, Engine};

#[derive(Debug)]
pub struct SummariesRequest {
    api_key: String,
}

impl SummariesRequest {
    pub fn new(api_key: impl ToString) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }
}

impl SummariesRequest {
    pub fn get(&self) -> Result<Summaries, SummariesRequestError> {
        reqwest::blocking::Client::new()
            .request(
                reqwest::Method::GET,
                "https://wakatime.com/api/v1/users/current/summaries",
            )
            .header(
                reqwest::header::AUTHORIZATION,
                format!(
                    "Basic {}",
                    base64::engine::general_purpose::STANDARD.encode(&self.api_key)
                ),
            )
            .query(&[["range", "Today"]])
            .send()
            .map_err(SummariesRequestError::http_error)?
            .json::<Summaries>()
            .map_err(SummariesRequestError::parse_error)
    }
}

#[cfg(test)]
mod test {
    use super::SummariesRequest;

    #[ignore = "No wakatime api key"]
    #[test]
    fn test_get() {
        let res = SummariesRequest { api_key: "".into() }.get();

        assert!(res.is_ok(), "{:?}", res.unwrap_err());
        println!("{:?}", res.unwrap());
    }
}
