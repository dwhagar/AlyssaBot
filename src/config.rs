use std::path::PathBuf;
use std::fs;
use url::Url;
use serde::{Deserialize, Serialize}; // For JSON handling

#[derive(Serialize, Deserialize)]
struct Config {
    settings_file: PathBuf,
    discord_token: String,
    wiki_base_url: String,
    command_prefix: String,
    log_verbosity: u8,
}

impl Config {
    fn new(file_name: Option<&str>) -> Result<Self, std::io::Error> {
        let settings_file = match file_name {
            Some(name) => PathBuf::from(name),
            None => PathBuf::from("alyssa.json"), // Default
        };

        let config_str = std::fs::read_to_string(&settings_file)?;
        let config: Config = serde_json::from_str(&config_str)?;
        Ok(config)
    }

    fn validate_config_file(&self) -> Result<(), std::io::Error> {
        let path = Path::new(&self.settings_file);

        // 1. Check if the path exists
        if !path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Configuration file not found",
            ));
        }

        // 2. Check readability (only if the file exists)
        if !path.metadata()?.permissions().readonly() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Configuration file is not readable",
            ));
        }

        // 3. Attempt a test write (optional, but highly recommended)
        let mut test_file = fs::OpenOptions::new()
            .write(true)
            .open(&path)?;
        // Note: You might consider immediately deleting a test file, or writing to a known-temporary location

        Ok(())
    }

    fn load_config(&mut self) -> Result<(), std::io::Error> {
        let config_str = fs::read_to_string(&self.settings_file)?;
        let loaded_config: Config = serde_json::from_str(&config_str)?;

        // Update the current struct with data from the file
        *self = loaded_config;

        Ok(())
    }

    fn save_config(&self) -> Result<(), std::io::Error> {
        let config_json = serde_json::to_string_pretty(&self)?; // Serialize the Config data
        fs::write(&self.settings_file, config_json)?; // Write to the file
        Ok(())
    }

    fn get_discord_token(&self) -> &str {
        &self.discord_token
    }

    fn get_wiki_url(&self) -> &str {
        &self.wiki_base_url
    }

    fn get_command_prefix(&self) -> &str {
        &self.command_prefix
    }

    fn get_verbosity(&self) -> u8 {
        self.log_verbosity
    }

    // Setters
    fn set_wiki_url(&mut self, url: &str) -> Result<(), url::ParseError> {
        let parsed_url = Url::parse(url)?; // Try to parse the URL

        // Check if the URL conforms to your desired scheme (e.g., "https")
        if parsed_url.scheme() != "https" && parsed_url.scheme() != "http" {
            return Err(url::ParseError::RelativeUrlWithoutBase);
        }

        self.wiki_base_url = url.to_string();
        Ok(())
    }

    fn set_command_prefix(&mut self, prefix: &str) {
        self.command_prefix = prefix.to_string();
    }

    fn set_verbosity(&mut self, level: u8) -> Result<(), &'static str> {
        if level > 3 {
            return Err("Verbosity level must be between 0 and 3");
        }

        self.log_verbosity = level;
        Ok(())
    }
}