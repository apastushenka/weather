use clap::{Parser, Subcommand};
use time::{macros::format_description, Date};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List available providers
    Providers,

    /// Configure provider
    Configure {
        /// Provider name
        provider: String,
    },

    /// Get weather
    Get {
        /// Address
        address: String,

        /// Date in YYYY-MM-DD format, default now
        #[arg(value_parser = validate_date)]
        date: Option<Date>,
    },
}

fn validate_date(s: &str) -> Result<Date, String> {
    let format = format_description!("[year]-[month]-[day]");
    Date::parse(s, &format).map_err(|e| e.to_string())
}
