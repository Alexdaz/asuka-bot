use serde::{Deserialize, Serialize};
use std::{path::Path, process::exit};
use toml;

const CONFIG_FILE: &str = "Settings.toml";

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub settings: Settings
}

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub prefix: char,
    pub activity: String,
    pub timezone: String,
    pub debug: u8
}

fn create_config()
{
    let config: Data = Data {
        settings: Settings {
            prefix: '$',
            activity: "Napping...".to_string(),
            timezone: "America/New_York".to_string(),
            debug: 0,
        },
    };

    let toml_string: String = toml::to_string(&config).expect("Could not encode TOML value");
    std::fs::write(CONFIG_FILE, toml_string).expect("Could not write to file!");
}

pub fn load_config() -> Data 
{
    if !Path::new(CONFIG_FILE).exists()
    {
        create_config();
    }

    let contents: String = match std::fs::read_to_string(CONFIG_FILE) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", CONFIG_FILE);
            exit(1);
        }
    };

    match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", CONFIG_FILE);
            exit(1);
        }
    }
}