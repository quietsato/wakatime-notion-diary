use crate::summaries::{Summaries, SummariesRequestError};
use base64::{self, Engine};
use chrono::{DateTime, TimeZone, Timelike, Utc};

#[derive(Debug)]
pub struct SummariesRequest<Tz: TimeZone> {
    api_key: String,
    today: DateTime<Tz>,
}

impl<Tz: TimeZone> SummariesRequest<Tz> {
    pub fn new(api_key: impl ToString, today: DateTime<Tz>) -> Self {
        Self {
            api_key: api_key.to_string(),
            today,
        }
    }

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
            .query(&[
                [
                    "start",
                    &self
                        .today
                        .with_hour(0)
                        .and_then(|today| today.with_minute(0))
                        .and_then(|today| today.with_second(0))
                        .unwrap()
                        .with_timezone(&Utc)
                        .to_rfc3339(),
                ],
                [
                    "end",
                    &self
                        .today
                        .with_hour(23)
                        .and_then(|today| today.with_minute(59))
                        .and_then(|today| today.with_second(59))
                        .unwrap()
                        .with_timezone(&Utc)
                        .to_rfc3339(),
                ],
            ])
            .send()
            .map_err(SummariesRequestError::http_error)?
            .json::<Summaries>()
            .map_err(SummariesRequestError::parse_error)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Local;

    #[ignore]
    #[test]
    fn test_get() {
        let res = SummariesRequest {
            api_key: "".into(),
            today: Local::now(),
        }
        .get();

        assert!(res.is_ok(), "{:?}", res.unwrap_err());
        println!("{:?}", res.unwrap());
    }
}
