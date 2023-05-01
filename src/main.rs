use clap::Parser;
use once_cell::sync::Lazy;
use provider::{DummyProvider, VisualCrossingProvider, WeatherProvider};
use std::collections::BTreeMap;

mod cli;
mod config;
mod provider;

static PROVIDERS: Lazy<BTreeMap<&str, &str>> = Lazy::new(|| {
    let mut providers = BTreeMap::new();
    providers.insert(DummyProvider::name(), DummyProvider::description());
    providers.insert(
        VisualCrossingProvider::name(),
        VisualCrossingProvider::description(),
    );

    providers
});

fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Providers => {
            println!("Available providers:");
            for (name, desc) in PROVIDERS.iter() {
                println!("  {:10} {}", name, desc);
            }
        }

        _ => {}
    }
}
