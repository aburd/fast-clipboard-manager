use crate::store::Key;
use log::info;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub config_dir: PathBuf,
    pub clipboard_size: usize,
    key_path: Option<PathBuf>,
}

impl Config {
    /// Loads config file from the dir_path with config.json appended
    /// Creates file if it doesn't exist
    pub fn load(dir_path: PathBuf) -> Result<Config, Box<dyn Error>> {
        info!("Loading config from: {:?}", dir_path);
        if !Path::new(&dir_path).exists() {
            info!("Path [{:?}] does not exist, creating...", dir_path);
            fs::create_dir(&dir_path)?;
        }

        let config_path = dir_path.join(CONFIG_FILE_NAME);
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&config_path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        if buffer.is_empty() {
            info!("No config file found. Creating at {:?}.", &config_path);
            let config = Config::new(&dir_path, 100);
            buffer = serde_json::to_string(&config)?;
            f.write_all(&buffer.as_bytes())?;
        }
        Ok(serde_json::from_str::<Config>(&buffer)?)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let bytes = serde_json::to_vec(self)?;
        let mut f = OpenOptions::new()
            .write(true)
            .open(self.config_dir.join(CONFIG_FILE_NAME))?;

        f.write_all(&bytes)?;
        Ok(())
    }

    pub fn get_key(&self) -> Result<Key, Box<dyn Error>> {
        let key_path = self.key_path.as_ref().unwrap();
        let mut f = OpenOptions::new().read(true).open(key_path)?;
        let mut buf = vec![];
        f.read_to_end(&mut buf)?;
        return Ok(buf
            .try_into()
            .expect("Could not convert buffer into key array"));
    }

    pub fn update_key_path(&mut self, path: PathBuf) {
        self.key_path = Some(path);
    }

    fn new(p: &PathBuf, clipboard_size: usize) -> Config {
        Config {
            config_dir: p.clone(),
            clipboard_size,
            key_path: None,
        }
    }
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let home_path = home::home_dir().unwrap();
    let dir_path = home_path.join(".config/fast_clipboard_manager");
    Config::load(dir_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, OpenOptions};

    fn new_file() -> (File, PathBuf) {
        let tmp_file = temp_file::empty();
        let path_buf = PathBuf::from(tmp_file.path());
        let config = Config::new(&path_buf, 5);
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path_buf)
            .unwrap();
        let json = serde_json::to_vec(&config).unwrap();
        file.write_all(&json).unwrap();
        (file, path_buf)
    }

    #[test]
    fn test_config_can_load() {
        let (_file, path_buf) = new_file();
        let config = Config::load(path_buf.clone()).unwrap();
        assert_eq!(config.config_dir.to_str(), path_buf.to_str());
    }

    #[test]
    fn test_config_can_save() {
        let (_file, path_buf) = new_file();
        {
            let mut config = Config::load(path_buf.clone()).unwrap();
            config.clipboard_size = 300;
            config.save().unwrap();
        }
        let config = Config::load(path_buf.clone()).unwrap();
        assert_eq!(config.clipboard_size, 300);
    }
}
