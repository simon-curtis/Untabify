use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::read_to_string,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(skip)]
    config_path: String,
    tab_sizes: HashMap<String, usize>,
}

impl Default for Config {
    fn default() -> Self {
        let mut tab_sizes = HashMap::new();
        tab_sizes.insert("default".to_string(), 4);
        tab_sizes.insert("sql".to_string(), 5);
        tab_sizes.insert("xml".to_string(), 2);

        Config {
            config_path: ConfigDefaults::config_path().to_str().unwrap().to_string(),
            tab_sizes,
        }
    }
}

impl Config {
    pub fn save(&self) -> Self {
        let _ = std::fs::write(
            self.config_path.as_str(),
            serde_json::to_string_pretty(&self).unwrap(),
        );
        self.clone()
    }

    pub fn config_path(&self) -> &str {
        &self.config_path
    }

    pub fn get_tab_size(&self, extension: &str) -> &usize {
        match self.tab_sizes.get(extension) {
            Some(v) => v,
            None => self.tab_sizes.get("default").unwrap_or(&4),
        }
    }

    pub fn set_tab_size(&mut self, tab_size: &usize, extension: Option<&str>) {
        let extension = extension.unwrap_or("default").to_lowercase().to_string();
        self.tab_sizes.insert(extension, *tab_size);
        self.save();
    }

    pub fn load<P: AsRef<Path>>(path: &P) -> Config {
        let path = path.as_ref();

        if let Some(config_folder) = path.parent() {
            std::fs::create_dir_all(&config_folder).expect("Failed to create config directory");
        }

        if !path.exists() {
            let config = Config::default();
            config.save()
        } else {
            let config_str = read_to_string(path).expect("Failed to read config file");
            serde_json::from_str(&config_str).expect("Failed to parse config file")
        }
    }
}

pub struct ConfigDefaults;

impl ConfigDefaults {
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap()
            .join("untabify")
            .join("config.json")
    }
}