use log::info;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Config {
    config_dir: PathBuf,
    clipboard_size: usize,
}

impl Config {
    /// Loads config file from the dir_path with config.json appended
    /// Creates file if it doesn't exist
    pub fn load(dir_path: PathBuf) -> Result<Config, Box<dyn Error>> {
        if !Path::new(&dir_path).exists() {
            info!("Path [{:?}] does not exist, creating...", dir_path);
            fs::create_dir(&dir_path)?;
        }
        let config_path = dir_path.join("config.json");
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&config_path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        if buffer.is_empty() {
            info!("No config file found. Creating at {:?}.", &config_path);
            let serialized = serde_json::to_vec(&Config::new(&config_path, 100))?;
            f.write_all(&serialized)?;
        }
        Ok(serde_json::from_str::<Config>(&buffer)?)
    }

    fn new(p: &PathBuf, clipboard_size: usize) -> Config {
        Config {
            config_dir: p.clone(),
            clipboard_size,
        }
    }
}
