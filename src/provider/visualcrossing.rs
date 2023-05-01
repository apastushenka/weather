//! [Visual Crossing](https://www.visualcrossing.com) weather provider

use super::{Error, Temperature, WeatherProvider, WeatherReport};
use serde::{Deserialize, Serialize};
use time::macros::format_description;

const BASE_URL: &str =
    "https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline";

#[derive(Serialize, Deserialize)]
pub struct VisualCrossingProvider {
    base_url: String,
    key: String,
}

impl VisualCrossingProvider {
    pub fn new(key: String) -> Self {
        Self {
            base_url: BASE_URL.into(),
            key,
        }
    }
}

#[typetag::serde]
impl WeatherProvider for VisualCrossingProvider {
    fn name() -> &'static str {
        "vc"
    }

    fn description() -> &'static str {
        "Visual Crossing weather provider"
    }

    fn get_weather(&self, location: &str, date: time::Date) -> Result<WeatherReport, Error> {
        let date = date
            .format(format_description!("[year]-[month]-[day]"))
            .unwrap();

        let path = format!("{}/{}/{}", self.base_url, location, date);
        let response = ureq::get(&path)
            .query("key", &self.key)
            .query("unitGroup", "metric")
            .query("include", "days")
            .query("elements", "datetime,temp,conditions,description")
            .call();

        match response {
            Ok(response) => {
                let report = response
                    .into_json::<VisualCrossingReport>()
                    .map_err(Box::<dyn std::error::Error>::from)?
                    .try_into()?;

                Ok(report)
            }
            Err(ureq::Error::Status(400, response)) => {
                let message = response
                    .into_string()
                    .map_err(Box::<dyn std::error::Error>::from)?;

                Err(Error::ApiError(message))
            }
            Err(ureq::Error::Status(401, _)) => Err(Error::AuthError),
            Err(e) => Err(Box::<dyn std::error::Error>::from(e).into()),
        }
    }
}

#[derive(Deserialize)]
struct VisualCrossingDayReport {
    temp: f32,
    conditions: String,
    description: String,
}

#[derive(Deserialize)]
struct VisualCrossingReport {
    days: Vec<VisualCrossingDayReport>,
}

impl TryFrom<VisualCrossingReport> for WeatherReport {
    type Error = Error;

    fn try_from(value: VisualCrossingReport) -> Result<Self, Self::Error> {
        if let Some(day) = value.days.into_iter().next() {
            let condition = if day.description.is_empty() {
                day.conditions
            } else {
                format!("{} ({})", day.conditions, day.description)
            };

            Ok(WeatherReport {
                temperature: Temperature::C(day.temp),
                condition,
            })
        } else {
            Err(Error::ApiError("no weather data in response".into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use httpmock::{Method::GET, MockServer};
    use time::macros::date;

    fn provider(server: &MockServer) -> VisualCrossingProvider {
        VisualCrossingProvider {
            base_url: server.base_url(),
            key: "SECRET".into(),
        }
    }

    #[test]
    fn get_weather() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/Minsk/2023-05-01");
            then.status(200).body(
                r#"{
                "days": [
                    {
                        "temp": 7.7,
                        "conditions": "Clear",
                        "description": "Clear conditions throughout the day."
                    }
                ]
                }"#,
            );
        });

        let report = provider(&server)
            .get_weather("Minsk", date!(2023 - 05 - 01))
            .unwrap();
        assert_matches!(report.temperature, Temperature::C(v) => assert_eq!(v, 7.7));
        assert_eq!(
            report.condition,
            "Clear (Clear conditions throughout the day.)"
        );

        mock.assert();
    }

    #[test]
    fn get_weather_invalid_key() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/Minsk/2023-05-01");
            then.status(401)
                .body("No account found with API key 'SECRET'");
        });

        let err = provider(&server)
            .get_weather("Minsk", date!(2023 - 05 - 01))
            .unwrap_err();
        assert_matches!(err, Error::AuthError);

        mock.assert();
    }

    #[test]
    fn get_weather_invalid_location() {
        let server = MockServer::start();

        let body = "Invalid location found. Please check your location parameter".to_string();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/INVALID_LOCATION/2023-05-01");
            then.status(400).body(&body);
        });

        let err = provider(&server)
            .get_weather("INVALID_LOCATION", date!(2023 - 05 - 01))
            .unwrap_err();
        assert_matches!(err, Error::ApiError(value) => assert_eq!(value, body));

        mock.assert();
    }

    #[test]
    fn get_weather_invalid_date() {
        let server = MockServer::start();

        let body = "Invalid year requested. Years must be between 1950 and 2050".to_string();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/Minsk/1900-01-01");
            then.status(400).body(&body);
        });

        let err = provider(&server)
            .get_weather("Minsk", date!(1900 - 01 - 01))
            .unwrap_err();
        assert_matches!(err, Error::ApiError(value) => assert_eq!(value, body));

        mock.assert();
    }

    #[test]
    fn get_weather_server_error() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/Minsk/2023-05-01");
            then.status(500);
        });

        let err = provider(&server)
            .get_weather("Minsk", date!(2023 - 05 - 01))
            .unwrap_err();
        assert_matches!(err, Error::Other(_));

        mock.assert();
    }
}
