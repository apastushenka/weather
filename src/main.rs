use provider::{dummy::DummyProvider, WeatherProvider};

mod provider;

fn main() {
    let source: Box<dyn WeatherProvider> = Box::new(DummyProvider::new());
    let report = source.get_weather("", time::Date::MIN).unwrap();

    println!("{}", report);
}
