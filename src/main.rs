use anyhow::{bail, Context, Result};
use clap::Parser;
use once_cell::sync::Lazy;
use provider::{DummyProvider, VisualCrossingProvider, WeatherProvider};
use std::{collections::BTreeMap, fs::File, path::PathBuf};

mod cli;
mod config;
mod provider;

type ProviderBuilder = fn() -> Box<dyn WeatherProvider>;

static PROVIDERS: Lazy<BTreeMap<&str, (&str, ProviderBuilder)>> = Lazy::new(|| {
    let mut providers = BTreeMap::<&str, (&str, ProviderBuilder)>::new();

    providers.insert(
        DummyProvider::name(),
        (DummyProvider::description(), || {
            Box::new(DummyProvider::new())
        }),
    );

    providers.insert(
        VisualCrossingProvider::name(),
        (VisualCrossingProvider::description(), || {
            println!("Enter API key:");

            let mut key = String::new();
            std::io::stdin().read_line(&mut key).expect("valid string");

            Box::new(VisualCrossingProvider::new(key.trim().to_owned()))
        }),
    );

    providers
});

fn config_path() -> std::io::Result<PathBuf> {
    let mut path = std::env::current_exe()?;
    path.set_extension("json");
    Ok(path)
}

fn save(provider: Box<dyn WeatherProvider>) -> Result<()> {
    let path = config_path()?;
    let file = File::create(path)?;
    config::save(&file, provider)?;

    Ok(())
}

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Providers => {
            println!("Available providers:");
            for (name, (desc, _)) in PROVIDERS.iter() {
                println!("  {:10} {}", name, desc);
            }
        }

        cli::Commands::Configure { provider: name } => {
            if let Some((_, build)) = PROVIDERS.get(name.as_str()) {
                let provider = build();
                save(provider).context("Failed to save provider")?;
            } else {
                bail!("no such provider: {}", name);
            }
        }
        _ => {}
    }

    Ok(())
}
