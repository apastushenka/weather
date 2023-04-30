pub mod dummy;

#[derive(Debug)]
pub enum Temperature {
    /// The temperature in Celsius
    C(f32),
}

impl std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Temperature::C(value) => write!(f, "{}C", value),
        }
    }
}

#[derive(Debug)]
pub struct WeatherReport {
    pub temperature: Temperature,
    pub condition: String,
}

impl std::fmt::Display for WeatherReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.temperature, self.condition)
    }
}

pub trait WeatherProvider {
    /// Returns a weather for given location and date
    fn get_weather(
        &self,
        location: &str,
        date: time::Date,
    ) -> Result<WeatherReport, Box<dyn std::error::Error>>;
}
