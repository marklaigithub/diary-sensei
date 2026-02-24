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
pub async fn search_entries(query: String) -> Result<Vec<EntryListItem>, String> {
    storage::search_entries(&query)
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
    created_at: Option<String>,
) -> Result<String, String> {
    let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let is_new = id.is_none();
    let entry_id = id.unwrap_or_else(|| storage::generate_entry_id(&date));

    let final_created_at = if is_new {
        Some(now.clone())
    } else {
        // Preserve existing created_at, or derive from id for old entries
        created_at.or_else(|| {
            // Derive from id: "2026-02-24_143052" -> "2026-02-24T14:30:52"
            let parts: Vec<&str> = entry_id.splitn(2, '_').collect();
            if parts.len() == 2 && parts[1].len() >= 6 {
                let t = parts[1];
                Some(format!("{}T{}:{}:{}", parts[0], &t[0..2], &t[2..4], &t[4..6]))
            } else {
                None
            }
        })
    };

    let entry = DiaryEntry {
        meta: EntryMeta {
            id: entry_id.clone(),
            date,
            title,
            mode,
            languages,
            date_format,
            created_at: final_created_at,
            updated_at: Some(now),
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

#[derive(serde::Serialize)]
pub struct CorrectionResult {
    pub corrected: String,
    pub explanation: Option<String>,
}

#[command]
pub async fn correct_text(text: String, language: String) -> Result<CorrectionResult, String> {
    let config = config::load_app_config();
    let lang_name = config
        .languages
        .iter()
        .find(|l| l.code == language)
        .map(|l| l.name.clone())
        .unwrap_or(language);
    let system = claude::correction_prompt(&lang_name);

    let raw = match config.ai_provider.as_str() {
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

    // Parse [CORRECTED] / [EXPLANATION] sections
    let (corrected, explanation) = if let Some(corr_start) = raw.find("[CORRECTED]") {
        let after_marker = &raw[corr_start + "[CORRECTED]".len()..];
        if let Some(expl_start) = after_marker.find("[EXPLANATION]") {
            let corrected = after_marker[..expl_start].trim().to_string();
            let explanation = after_marker[expl_start + "[EXPLANATION]".len()..].trim().to_string();
            (corrected, if explanation.is_empty() { None } else { Some(explanation) })
        } else {
            (after_marker.trim().to_string(), None)
        }
    } else {
        // Fallback: treat entire response as corrected text (backward compat)
        (raw.trim().to_string(), None)
    };

    Ok(CorrectionResult { corrected, explanation })
}

#[command]
pub async fn translate_text(
    text: String,
    target_languages: Vec<String>,
) -> Result<HashMap<String, String>, String> {
    let config = config::load_app_config();

    if config.ai_provider == "claude" && config.api_key.is_empty() {
        return Err("API key not configured. Please set it in Settings.".to_string());
    }

    // Spawn all translations in parallel
    let futures: Vec<_> = target_languages.iter().map(|lang_code| {
        let lang_name = config
            .languages
            .iter()
            .find(|l| &l.code == lang_code)
            .map(|l| l.name.clone())
            .unwrap_or_else(|| lang_code.clone());
        let system = claude::translation_prompt(&lang_name);
        let text = text.clone();
        let api_key = config.api_key.clone();
        let provider = config.ai_provider.clone();
        let ollama_url = config.ollama_url.clone();
        let ollama_model = config.ollama_model.clone();
        let lang_code = lang_code.clone();

        async move {
            let result = match provider.as_str() {
                "claude" => claude::call_claude(&api_key, &system, &text).await,
                _ => claude::call_ollama(&ollama_url, &ollama_model, &system, &text).await,
            };
            (lang_code, result)
        }
    }).collect();

    let results_vec = futures::future::join_all(futures).await;
    let mut results = HashMap::new();
    for (lang_code, result) in results_vec {
        match result {
            Ok(text) => { results.insert(lang_code, text); }
            Err(e) => { results.insert(lang_code, format!("[Translation failed: {}]", e)); }
        }
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
