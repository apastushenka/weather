//! Just a dummy weather provider

use super::{Temperature, WeatherProvider, WeatherReport};

pub struct DummyProvider;

impl DummyProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl WeatherProvider for DummyProvider {
    fn get_weather(
        &self,
        _location: &str,
        _date: time::Date,
    ) -> Result<WeatherReport, Box<dyn std::error::Error>> {
        Ok(WeatherReport {
            temperature: Temperature::C(20.5),
            condition: "Sunny".into(),
        })
    }
}
