use std::fs::{read_to_string, write};
use std::path::PathBuf;
use toml::{to_string, from_str};
use serde::{Deserialize, Serialize};

// Struct of all the config options
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub bot_token: String,
    pub prefixes: Vec<String>,
}

impl Default for Config {
    // Create default function called by serde if values are missing from config
    fn default() -> Self {
        Self {
            bot_token: "XXXXXX".to_string(),
            prefixes: Vec::from([
                "🥺".to_string(),
                "What the fuck is".to_string(),
                "use your words".to_string(),
                "I don't speak bottom".to_string(),
                "I dont speak bottom".to_string(),
            ]),
        }
    }
}

impl Config {
    pub fn create() -> String {
        let path = PathBuf::from("./config.toml");
        let config = Self { ..Default::default() };
        
        let out = to_string(&config).expect("Failed to convert to TOML format");
        write(path, &out).expect("Failed to write config.toml");
        out
    }

    pub fn get() -> Self {
        let path = PathBuf::from("./config.toml");
        let file = match read_to_string(&path) {
            Ok(f) => {
                f
            },
            // If file does not exist
            Err(e) => {
                println!("{}\nNo config file found! Creating a new file, please configure it with your bot token", e);
                Config::create()
            },
        };
        
        // Load config from file, injecting defaults if values are missing
        let conf = from_str(&file).expect("Failed to parse TOML");
        // Rewrite config to file, this creates values that were missing, if any
        write(path, to_string(&conf).unwrap()).expect("Failed to write config.toml");
        conf
    }
}