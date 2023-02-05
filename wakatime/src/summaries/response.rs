mod data;
mod grand_total;
mod language;
mod projects;

use self::data::SummariesData;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Summaries {
    pub data: Vec<SummariesData>,
}

#[cfg(test)]
mod test {
    use crate::summaries::{
        response::{
            data::SummariesData, grand_total::GrandTotal, language::Language, projects::Project,
        },
        Summaries,
    };

    #[test]
    fn test_parse_response() {
        const JSON_STR: &str = r#"
        {
            "cumulative_total": {
              "decimal": "1.22",
              "digital": "1:13",
              "seconds": 4429.475073,
              "text": "1 hr 13 mins"
            },
            "daily_average": {
              "days_including_holidays": 1,
              "days_minus_holidays": 1,
              "holidays": 0,
              "seconds": 4429,
              "seconds_including_other_language": 4429,
              "text": "1 hr 13 mins",
              "text_including_other_language": "1 hr 13 mins"
            },
            "data": [
              {
                "categories": [
                  {
                    "decimal": "1.20",
                    "digital": "1:12:42",
                    "hours": 1,
                    "minutes": 12,
                    "name": "Coding",
                    "percent": 98.49,
                    "seconds": 42,
                    "text": "1 hr 12 mins",
                    "total_seconds": 4362.781144
                  },
                  {
                    "decimal": "0.02",
                    "digital": "0:01:06",
                    "hours": 0,
                    "minutes": 1,
                    "name": "Building",
                    "percent": 1.51,
                    "seconds": 6,
                    "text": "1 min",
                    "total_seconds": 66.693929
                  }
                ],
                "dependencies": [],
                "editors": [
                  {
                    "decimal": "1.22",
                    "digital": "1:13:49",
                    "hours": 1,
                    "minutes": 13,
                    "name": "Editor",
                    "percent": 100.0,
                    "seconds": 49,
                    "text": "1 hr 13 mins",
                    "total_seconds": 4429.475073
                  }
                ],
                "grand_total": {
                  "decimal": "1.22",
                  "digital": "1:13",
                  "hours": 1,
                  "minutes": 13,
                  "text": "1 hr 13 mins",
                  "total_seconds": 4429.475073
                },
                "languages": [
                  {
                    "decimal": "1.22",
                    "digital": "1:13:34",
                    "hours": 1,
                    "minutes": 13,
                    "name": "Rust",
                    "percent": 99.66,
                    "seconds": 34,
                    "text": "1 hr 13 mins",
                    "total_seconds": 4414.490516
                  },
                  {
                    "decimal": "0.00",
                    "digital": "0:00:14",
                    "hours": 0,
                    "minutes": 0,
                    "name": "TOML",
                    "percent": 0.32,
                    "seconds": 14,
                    "text": "14 secs",
                    "total_seconds": 14.303398
                  },
                  {
                    "decimal": "0.00",
                    "digital": "0:00:00",
                    "hours": 0,
                    "minutes": 0,
                    "name": "Other",
                    "percent": 0.02,
                    "seconds": 0,
                    "text": "0 secs",
                    "total_seconds": 0.681159
                  }
                ],
                "machines": [],
                "operating_systems": [],
                "projects": [
                  {
                    "color": null,
                    "decimal": "1.22",
                    "digital": "1:13:49",
                    "hours": 1,
                    "minutes": 13,
                    "name": "sample",
                    "percent": 100.0,
                    "seconds": 49,
                    "text": "1 hr 13 mins",
                    "total_seconds": 4429.475073
                  }
                ],
                "range": {
                  "date": "2023-02-05",
                  "end": "2023-02-05T14:59:59Z",
                  "start": "2023-02-04T15:00:00Z",
                  "text": "Sun Feb 5th 2023",
                  "timezone": "Asia/Tokyo"
                }
              }
            ],
            "end": "2023-02-05T14:59:59Z",
            "start": "2023-02-04T15:00:00Z"
          }
        "#;

        let parsed = serde_json::from_str::<Summaries>(JSON_STR);
        assert!(parsed.is_ok(), "{:?}", parsed.unwrap_err());

        let parsed = parsed.unwrap();
        assert_eq!(
            parsed,
            Summaries {
                data: vec![SummariesData {
                    grand_total: GrandTotal {
                        digital: "1:13".into(),
                        hours: 1,
                        minutes: 13,
                        text: "1 hr 13 mins".into(),
                        total_seconds: 4_429.475
                    },
                    languages: vec![
                        Language {
                            name: "Rust".into(),
                            total_seconds: 4414.4907,
                            percent: 99.66,
                            digital: "1:13:34".into(),
                            text: "1 hr 13 mins".into(),
                            hours: 1,
                            minutes: 13,
                            seconds: 34
                        },
                        Language {
                            name: "TOML".into(),
                            total_seconds: 14.303398,
                            percent: 0.32,
                            digital: "0:00:14".into(),
                            text: "14 secs".into(),
                            hours: 0,
                            minutes: 0,
                            seconds: 14
                        },
                        Language {
                            name: "Other".into(),
                            total_seconds: 0.681159,
                            percent: 0.02,
                            digital: "0:00:00".into(),
                            text: "0 secs".into(),
                            hours: 0,
                            minutes: 0,
                            seconds: 0
                        },
                    ],
                    projects: vec![Project {
                        name: "sample".into(),
                        total_seconds: 4_429.475,
                        percent: 100.0,
                        digital: "1:13:49".into(),
                        text: "1 hr 13 mins".into(),
                        hours: 1,
                        minutes: 13,
                        seconds: 49
                    }]
                }]
            },
        );
    }
}
