use super::{SummariesRequestError, SummariesResponse};
use base64::{self, Engine};

#[derive(Debug)]
pub struct SummariesRequest {
    pub api_key: String,
}

impl SummariesRequest {
    fn get(&self) -> Result<SummariesResponse, SummariesRequestError> {
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
            .json::<SummariesResponse>()
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
