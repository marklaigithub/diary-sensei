use tauri::command;
use std::collections::HashMap;
use crate::claude;
use crate::config::{self, AppConfig};
use crate::storage::{self, DiaryEntry, EntryListItem, EntryMeta};

#[command]
pub async fn list_entries(year: i32, month: u32) -> Result<Vec<EntryListItem>, String> {
    storage::list_entries_for_month(year, month)
}

#[command]
pub async fn read_entry(id: String) -> Result<DiaryEntry, String> {
    storage::read_entry_by_id(&id)
}

#[command]
pub async fn save_entry(
    id: Option<String>,
    title: String,
    date: String,
    mode: String,
    languages: Vec<String>,
    original: String,
    translations: HashMap<String, String>,
    date_format: Option<String>,
) -> Result<String, String> {
    let entry_id = id.unwrap_or_else(|| storage::generate_entry_id(&date));
    let entry = DiaryEntry {
        meta: EntryMeta {
            id: entry_id.clone(),
            date,
            title,
            mode,
            languages,
            date_format,
        },
        original,
        translations,
    };
    storage::save_entry_to_disk(&entry)?;
    Ok(entry_id)
}

#[command]
pub async fn save_image(id: String, filename: String, data: Vec<u8>) -> Result<String, String> {
    storage::save_image_to_disk(&id, &filename, &data)
}

#[command]
pub async fn delete_entry(id: String) -> Result<(), String> {
    storage::delete_entry_from_disk(&id)
}

#[command]
pub async fn create_entry_id(date: String) -> String {
    storage::generate_entry_id(&date)
}

#[command]
pub async fn correct_text(text: String, language: String) -> Result<String, String> {
    let config = config::load_app_config();
    let lang_name = config
        .languages
        .iter()
        .find(|l| l.code == language)
        .map(|l| l.name.clone())
        .unwrap_or(language);
    let system = claude::correction_prompt(&lang_name);

    match config.ai_provider.as_str() {
        "claude" => {
            if config.api_key.is_empty() {
                return Err("API key not configured. Please set it in Settings.".to_string());
            }
            claude::call_claude(&config.api_key, &system, &text).await
        }
        _ => {
            claude::call_ollama(&config.ollama_url, &config.ollama_model, &system, &text).await
        }
    }
}

#[command]
pub async fn translate_text(
    text: String,
    target_languages: Vec<String>,
) -> Result<HashMap<String, String>, String> {
    let config = config::load_app_config();
    let mut results = HashMap::new();

    for lang_code in &target_languages {
        let lang_name = config
            .languages
            .iter()
            .find(|l| &l.code == lang_code)
            .map(|l| l.name.clone())
            .unwrap_or_else(|| lang_code.clone());
        let system = claude::translation_prompt(&lang_name);

        let result = match config.ai_provider.as_str() {
            "claude" => {
                if config.api_key.is_empty() {
                    return Err("API key not configured. Please set it in Settings.".to_string());
                }
                claude::call_claude(&config.api_key, &system, &text).await?
            }
            _ => {
                claude::call_ollama(&config.ollama_url, &config.ollama_model, &system, &text).await?
            }
        };
        results.insert(lang_code.clone(), result);
    }

    Ok(results)
}

#[command]
pub async fn load_config() -> Result<AppConfig, String> {
    Ok(config::load_app_config())
}

#[command]
pub async fn save_config(config: AppConfig) -> Result<(), String> {
    config::save_app_config(&config)
}

#[command]
pub async fn get_entries_dir() -> Result<String, String> {
    let config = config::load_app_config();
    Ok(config.entries_dir)
}

#[command]
pub async fn print_page(webview_window: tauri::WebviewWindow) -> Result<(), String> {
    webview_window.print().map_err(|e| e.to_string())
}
