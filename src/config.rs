use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Config {
    #[serde(default = "default_aur_helper")]
    pub aur_helper: String,
    #[serde(default = "default_filter")]
    pub default_filter: String,
    #[serde(default = "default_sort")]
    pub default_sort: String,
}

#[allow(dead_code)]
fn default_aur_helper() -> String {
    "paru".to_string()
}

#[allow(dead_code)]
fn default_filter() -> String {
    "all".to_string()
}

#[allow(dead_code)]
fn default_sort() -> String {
    "name".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            aur_helper: default_aur_helper(),
            default_filter: default_filter(),
            default_sort: default_sort(),
        }
    }
}

/// Load configuration from ~/.config/glare/config.toml
/// Falls back to defaults if file doesn't exist or is malformed
pub fn load_config() -> Config {
    let config_path = get_config_path();
    if config_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            if let Ok(config) = toml::from_str(&content) {
                return config;
            }
        }
    }
    Config::default()
}

fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("glare")
        .join("config.toml")
}
