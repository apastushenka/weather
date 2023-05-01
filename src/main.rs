use clap::Parser;
use provider::{dummy::DummyProvider, WeatherProvider};

mod cli;
mod config;
mod provider;

fn main() {
    let cli = cli::Cli::parse();
}
