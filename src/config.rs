use crate::provider::WeatherProvider;
use anyhow::{Context, Result};
use std::io::{Read, Write};

pub fn save<W: Write>(writer: W, provider: Box<dyn WeatherProvider>) -> Result<()> {
    serde_json::to_writer(writer, &provider)?;
    Ok(())
}

pub fn load<R: Read>(reader: R) -> Option<Box<dyn WeatherProvider>> {
    serde_json::from_reader(reader).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::DummyProvider;

    #[test]
    fn load_empty_provider() {
        let res = load(std::io::empty());

        assert!(res.is_none())
    }

    #[test]
    fn load_provider() {
        let mut buffer = Vec::new();

        let res = save(&mut buffer, Box::new(DummyProvider::new()));
        assert!(res.is_ok());

        let res = load(&buffer[..]);
        assert!(res.is_some());
    }
}
