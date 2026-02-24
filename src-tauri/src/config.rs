use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub code: String,
    pub name: String,
    pub date_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub api_key: String,
    #[serde(default = "default_language")]
    pub default_language: String,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_languages")]
    pub languages: Vec<LanguageConfig>,
    #[serde(default = "default_entries_dir")]
    pub entries_dir: String,
    pub global_date_format: Option<String>,
    #[serde(default = "default_ai_provider")]
    pub ai_provider: String,
    #[serde(default = "default_ollama_model")]
    pub ollama_model: String,
    #[serde(default = "default_ollama_url")]
    pub ollama_url: String,
}

fn default_language() -> String {
    "ja".to_string()
}

fn default_theme() -> String {
    "warm-light".to_string()
}

fn default_ai_provider() -> String { "ollama".to_string() }
fn default_ollama_model() -> String { "gemma2:9b".to_string() }
fn default_ollama_url() -> String { "http://localhost:11434".to_string() }

fn default_entries_dir() -> String {
    dirs::document_dir()
        .unwrap_or_else(|| PathBuf::from("~"))
        .join("diary-language")
        .join("entries")
        .to_string_lossy()
        .to_string()
}

fn default_languages() -> Vec<LanguageConfig> {
    vec![
        LanguageConfig {
            code: "ja".to_string(),
            name: "日本語".to_string(),
            date_format: "YYYY年MM月DD日（ddd）".to_string(),
        },
        LanguageConfig {
            code: "en".to_string(),
            name: "English".to_string(),
            date_format: "MMM DD, YYYY (ddd)".to_string(),
        },
        LanguageConfig {
            code: "it".to_string(),
            name: "Italiano".to_string(),
            date_format: "DD/MM/YYYY (ddd)".to_string(),
        },
    ]
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            default_language: default_language(),
            theme: default_theme(),
            languages: default_languages(),
            entries_dir: default_entries_dir(),
            global_date_format: None,
            ai_provider: default_ai_provider(),
            ollama_model: default_ollama_model(),
            ollama_url: default_ollama_url(),
        }
    }
}

pub fn config_path() -> PathBuf {
    dirs::document_dir()
        .unwrap_or_else(|| PathBuf::from("~"))
        .join("diary-language")
        .join("config.json")
}

pub fn load_app_config() -> AppConfig {
    let path = config_path();
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        AppConfig::default()
    }
}

pub fn save_app_config(config: &AppConfig) -> Result<(), String> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}
