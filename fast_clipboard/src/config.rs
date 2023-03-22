use crate::store::Key;
use log::info;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const CONFIG_FILE_NAME: &str = "config.json";
const DEFAULT_CLIPBOARD_SIZE: usize = 5;

trait Storage {
    fn load(&mut self) -> anyhow::Result<()>;
    fn save(&self) -> anyhow::Result<()>;
}

pub struct ConfigFile {
    pub path: PathBuf,
    pub config: Config,
}

impl ConfigFile {
    pub fn new(path: &PathBuf) -> Self {
        ConfigFile {
            path: path.clone(),
            config: Config::default(),
        }
    }
}

impl Storage for ConfigFile {
    /// Loads config file from the dir_path with config.json appended
    /// Creates file if it doesn't exist
    fn load(&mut self) -> anyhow::Result<()> {
        let dir_path = self.path.parent().unwrap();
        info!("Loading config from: {:?}", dir_path);
        if !Path::new(&dir_path).exists() {
            info!("Path [{:?}] does not exist, creating...", dir_path);
            fs::create_dir_all(&dir_path)?;
        }

        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        if buffer.is_empty() {
            info!("No config file found. Creating at {:?}.", &self.path);
            buffer = serde_json::to_string(&self.config)?;
            f.write_all(&buffer.as_bytes())?;
        }
        let config = serde_json::from_str::<Config>(&buffer)?;
        self.config = config;
        Ok(())
    }

    fn save(&self) -> anyhow::Result<()> {
        let bytes = serde_json::to_vec(&self.config)?;
        let mut f = OpenOptions::new().write(true).open(&self.path)?;

        f.write_all(&bytes)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub clipboard_size: usize,
    key_path: Option<PathBuf>,
}

impl Config {
    pub fn get_key(&self) -> anyhow::Result<Key> {
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

    fn new(clipboard_size: usize) -> Config {
        Config {
            clipboard_size,
            key_path: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            clipboard_size: DEFAULT_CLIPBOARD_SIZE,
            key_path: None,
        }
    }
}

pub fn get_config() -> anyhow::Result<ConfigFile> {
    let home_path = home::home_dir().unwrap();
    let dir_path = home_path.join(".config/fast_clipboard_manager");
    let default_path = dir_path.join(&CONFIG_FILE_NAME);
    let mut config_file = ConfigFile::new(&default_path);
    config_file.load()?;
    Ok(config_file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, OpenOptions};

    fn new_file() -> (File, PathBuf) {
        let tmp_file = temp_file::empty();
        let path_buf = PathBuf::from(tmp_file.path());
        let mut config_file = ConfigFile::new(&path_buf);
        config_file.load().unwrap();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path_buf)
            .unwrap();
        let json = serde_json::to_vec(&config_file.config).unwrap();
        file.write_all(&json).unwrap();
        (file, path_buf)
    }

    #[test]
    fn test_config_can_load() {
        let (_file, path_buf) = new_file();
        let mut config_file = ConfigFile::new(&path_buf);
        config_file.load().unwrap();
        assert_eq!(config_file.path.to_str(), path_buf.to_str());
    }

    #[test]
    fn test_config_can_save() {
        let (_file, path_buf) = new_file();
        {
            let mut config_file = ConfigFile::new(&path_buf);
            config_file.load().unwrap();
            config_file.config.clipboard_size = 300;
            config_file.save().unwrap();
        }
        let mut config_file = ConfigFile::new(&path_buf);
        config_file.load().unwrap();
        assert_eq!(config_file.config.clipboard_size, 300);
    }
}
