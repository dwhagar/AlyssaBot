mod log;
mod config;
use std::env;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sets the verbosity level
    #[arg(short, long, value_name = "LEVEL")]
    verbosity: Option<u8>,

    /// Path to the configuration file
    #[arg(short, long)] // Note: No 'value_name' is needed here
    config: Option<String>,
}

fn main() {
    let args = Args::parse(); // Parse the arguments using 'clap'

    // Access parsed options
    let verbosity_level = args.verbosity.unwrap_or(0);  // Default to verbosity level '0'
    let config_file = args.config.unwrap_or_else(|| "alyssa.json".to_string());

    // Create your Config object
    let mut config = Config::new(Some(&config_file))?;

    // Override verbosity if set from command line
    if let Some(level) = args.verbosity {
        config.set_verbosity(level)?;
    }

    // ... (rest of your program logic)
}