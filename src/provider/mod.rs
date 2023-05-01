use thiserror::Error;

pub mod dummy;
pub mod visualcrossing;

pub use dummy::DummyProvider;
pub use visualcrossing::VisualCrossingProvider;

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

#[derive(Debug, Error)]
pub enum Error {
    #[error("authentication error")]
    AuthError,
    #[error("external API error: {0}")]
    ApiError(String),
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error>),
}

#[typetag::serde(tag = "type")]
pub trait WeatherProvider {
    /// Short name of provider
    fn name() -> &'static str
    where
        Self: Sized;

    /// Description of provider
    fn description() -> &'static str
    where
        Self: Sized;

    /// Returns a weather for given location and date
    fn get_weather(&self, location: &str, date: time::Date) -> Result<WeatherReport, Error>;
}
