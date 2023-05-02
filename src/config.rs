use crate::provider::WeatherProvider;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Stores configured weather providers.
#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    default: Option<String>,
    providers: HashMap<String, Box<dyn WeatherProvider>>,
}

impl Config {
    /// Adds provider and sets it as default if it is the single one.
    pub fn add(&mut self, name: String, provider: Box<dyn WeatherProvider>) {
        if self.providers.is_empty() {
            self.default = Some(name.clone());
        }

        self.providers.insert(name, provider);
    }

    /// Returns default provider or `None` if there are no configured providers.
    pub fn get_default(&self) -> Option<&Box<dyn WeatherProvider>> {
        let name = self.default.as_ref()?;
        self.providers.get(name)
    }

    /// Sets default provider.
    pub fn set_default(&mut self, name: String) -> bool {
        if self.providers.contains_key(&name) {
            self.default = Some(name);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::{DummyProvider, VisualCrossingProvider, WeatherProvider};

    #[test]
    fn get_default_provider_from_empty_config() {
        let config = Config::default();

        assert!(config.get_default().is_none());
    }

    #[test]
    fn add_single_provider() {
        let mut config = Config::default();
        config.add(
            DummyProvider::name().to_owned(),
            Box::new(DummyProvider::new()),
        );

        assert_eq!(config.default, Some(DummyProvider::name().to_owned()));
        assert_eq!(config.providers.len(), 1);
    }

    #[test]
    fn add_multiple_providers() {
        let mut config = Config::default();
        config.add(
            DummyProvider::name().to_owned(),
            Box::new(DummyProvider::new()),
        );
        config.add(
            VisualCrossingProvider::name().to_owned(),
            Box::new(VisualCrossingProvider::new("SECRET".into())),
        );

        assert_eq!(config.default, Some(DummyProvider::name().to_owned()));
        assert_eq!(config.providers.len(), 2);
    }

    #[test]
    fn set_default_provider() {
        let mut config = Config::default();
        config.add(
            DummyProvider::name().to_owned(),
            Box::new(DummyProvider::new()),
        );
        config.add(
            VisualCrossingProvider::name().to_owned(),
            Box::new(VisualCrossingProvider::new("SECRET".into())),
        );

        let name = VisualCrossingProvider::name().to_owned();
        assert!(config.set_default(name.clone()));
        assert_eq!(config.default, Some(name));
    }

    #[test]
    fn set_default_provider_wrong_name() {
        let mut config = Config::default();

        assert!(!config.set_default("".to_string()));
    }
}
