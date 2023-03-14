use crate::clipboard::Key;
use log::info;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const DEFAULT_CONFIG_FILE_NAME: &str = "config.json";
const DEFAULT_CLIPBOARD_SIZE: usize = 5;
const DEFAULT_CHARACTERS_COUNT: i64 = 120;
const DEFAULT_VISIBLE_CONTENT_CHARACTERS_COUNT: i64 = 10;
const DEFAULT_CHARACTER_COUNT_VISIBLE: bool = true;
const DEFAULT_CAN_TOGGLE_CONTENT_VISIBLE: bool = true;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Config {
    // Where the config file is stored
    pub config_dir: PathBuf,
    // Total number of clipboard entries stored
    pub clipboard_size: usize,
    // Path to the key
    key_path: Option<PathBuf>,
    // If text, how many characters are shown total
    // includes obscured text (i.e. * character)
    pub characters_count: i64,
    // If text, how many characters are visible to user
    // positive = how many are plaintext
    // 0 and below have special meaning
    // 0 = all displayed as *
    // negative = all content in plaintext
    pub visible_content_characters_count: i64,
    // Display the count of characters in each entry?
    pub character_count_visible: bool,
    // Define whether user can press a key to reveal the content
    pub can_toggle_content_visible: bool,
}

impl Config {
    /// Loads config file from the dir_path
    /// Creates file if it doesn't exist
    pub fn load(dir_path: PathBuf) -> Result<Config, Box<dyn Error>> {
        info!("Loading config from: {:?}", dir_path);
        if !Path::new(&dir_path).exists() {
            info!("Path [{:?}] does not exist, creating...", dir_path);
            fs::create_dir(&dir_path)?;
        }

        let config_path = dir_path.join(DEFAULT_CONFIG_FILE_NAME);
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&config_path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        if buffer.is_empty() {
            info!("No config file found. Creating at {:?}.", &config_path);
            let config = Config::default().config_dir(&dir_path);
            buffer = serde_json::to_string(&config)?;
            f.write_all(&buffer.as_bytes())?;
        }
        Ok(serde_json::from_str::<Config>(&buffer)?)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let bytes = serde_json::to_vec(self)?;
        let mut f = OpenOptions::new()
            .write(true)
            .open(self.config_dir.join(DEFAULT_CONFIG_FILE_NAME))?;

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

    fn config_dir(mut self, config_dir: &PathBuf) -> Self {
        self.config_dir = config_dir.clone();
        self
    }

    fn default_home_dir() -> PathBuf {
        let home_path = home::home_dir().unwrap();
        home_path.join(".config/fast_clipboard_manager")
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            config_dir: Self::default_home_dir(),
            clipboard_size: DEFAULT_CLIPBOARD_SIZE,
            key_path: None,
            characters_count: DEFAULT_CHARACTERS_COUNT,
            visible_content_characters_count: DEFAULT_VISIBLE_CONTENT_CHARACTERS_COUNT,
            character_count_visible: DEFAULT_CHARACTER_COUNT_VISIBLE,
            can_toggle_content_visible: DEFAULT_CAN_TOGGLE_CONTENT_VISIBLE,
        }
    }
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    Config::load(Config::default_home_dir())
}
