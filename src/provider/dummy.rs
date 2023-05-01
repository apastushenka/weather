//! Just a dummy weather provider.

use super::{Error, Temperature, WeatherProvider, WeatherReport};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DummyProvider;

impl DummyProvider {
    pub fn new() -> Self {
        Self {}
    }
}

#[typetag::serde]
impl WeatherProvider for DummyProvider {
    fn name() -> &'static str {
        "dummy"
    }

    fn description() -> &'static str {
        "Dummy provider"
    }

    fn get_weather(&self, _location: &str, _date: time::Date) -> Result<WeatherReport, Error> {
        Ok(WeatherReport {
            temperature: Temperature::C(20.5),
            condition: "Sunny".into(),
        })
    }
}
