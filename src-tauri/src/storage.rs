use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::config::load_app_config;
use chrono::Local;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryMeta {
    pub id: String,           // "2026-02-24_143052"
    pub date: String,         // "2026-02-24"
    pub title: String,
    pub mode: String,         // "correction" or "translation"
    pub languages: Vec<String>, // target languages, e.g. ["ja", "en"]
    pub date_format: Option<String>,
    pub created_at: Option<String>,  // ISO 8601: "2026-02-24T14:30:52"
    pub updated_at: Option<String>,  // ISO 8601: "2026-02-24T14:30:52"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiaryEntry {
    pub meta: EntryMeta,
    pub original: String,
    pub translations: HashMap<String, String>, // lang_code -> result text
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryListItem {
    pub id: String,
    pub date: String,
    pub title: String,
    pub mode: String,
    pub languages: Vec<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

fn entries_dir() -> PathBuf {
    let config = load_app_config();
    let home = dirs::home_dir().unwrap_or_default();
    let dir = config.entries_dir.replace('~', &home.to_string_lossy());
    PathBuf::from(dir)
}

pub fn generate_entry_id(date: &str) -> String {
    let now = Local::now();
    format!("{}_{}", date, now.format("%H%M%S"))
}

pub fn list_entries_for_month(year: i32, month: u32) -> Result<Vec<EntryListItem>, String> {
    let dir = entries_dir()
        .join(format!("{:04}", year))
        .join(format!("{:02}", month));

    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut entries = vec![];
    let read_dir = fs::read_dir(&dir).map_err(|e| e.to_string())?;

    for entry in read_dir {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "md") {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Some(mut meta) = parse_frontmatter(&content) {
                    // Old entries may not have an id field — derive from filename stem
                    if meta.id.is_empty() {
                        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                            meta.id = stem.to_string();
                        }
                    }
                    entries.push(EntryListItem {
                        id: meta.id,
                        date: meta.date,
                        title: meta.title,
                        mode: meta.mode,
                        languages: meta.languages,
                        created_at: meta.created_at,
                        updated_at: meta.updated_at,
                    });
                }
            }
        }
    }

    entries.sort_by(|a, b| b.id.cmp(&a.id));
    Ok(entries)
}

pub fn search_entries(query: &str) -> Result<Vec<EntryListItem>, String> {
    let base = entries_dir();
    if !base.exists() {
        return Ok(vec![]);
    }

    let query_lower = query.to_lowercase();
    let mut results = vec![];

    // Walk through all year/month directories
    let years = fs::read_dir(&base).map_err(|e| e.to_string())?;
    for year_entry in years {
        let year_entry = year_entry.map_err(|e| e.to_string())?;
        let year_path = year_entry.path();
        if !year_path.is_dir() { continue; }

        let months = match fs::read_dir(&year_path) {
            Ok(m) => m,
            Err(_) => continue,
        };
        for month_entry in months {
            let month_entry = month_entry.map_err(|e| e.to_string())?;
            let month_path = month_entry.path();
            if !month_path.is_dir() { continue; }

            let files = match fs::read_dir(&month_path) {
                Ok(f) => f,
                Err(_) => continue,
            };
            for file_entry in files {
                let file_entry = file_entry.map_err(|e| e.to_string())?;
                let path = file_entry.path();
                if !path.extension().map_or(false, |ext| ext == "md") { continue; }

                if let Ok(content) = fs::read_to_string(&path) {
                    // Check if title or body contains the query
                    if content.to_lowercase().contains(&query_lower) {
                        if let Some(mut meta) = parse_frontmatter(&content) {
                            if meta.id.is_empty() {
                                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                                    meta.id = stem.to_string();
                                }
                            }
                            results.push(EntryListItem {
                                id: meta.id,
                                date: meta.date,
                                title: meta.title,
                                mode: meta.mode,
                                languages: meta.languages,
                                created_at: meta.created_at,
                                updated_at: meta.updated_at,
                            });
                        }
                    }
                }
            }
        }
    }

    results.sort_by(|a, b| b.id.cmp(&a.id));
    Ok(results)
}

pub fn read_entry_by_id(id: &str) -> Result<DiaryEntry, String> {
    // id format: "2026-02-24_143052"
    // date is the first 10 characters
    if id.len() < 10 {
        return Err("Invalid id format".to_string());
    }
    let date = &id[..10];
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return Err("Invalid date in id".to_string());
    }

    let path = entries_dir()
        .join(parts[0])
        .join(parts[1])
        .join(format!("{}.md", id));

    if !path.exists() {
        return Err(format!("Entry not found: {}", id));
    }

    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    parse_entry(&content)
}

pub fn save_entry_to_disk(entry: &DiaryEntry) -> Result<(), String> {
    let id = &entry.meta.id;
    let date = &entry.meta.date;
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return Err("Invalid date format".to_string());
    }

    let dir = entries_dir().join(parts[0]).join(parts[1]);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let path = dir.join(format!("{}.md", id));
    let content = serialize_entry(entry);
    fs::write(&path, content).map_err(|e| e.to_string())
}

pub fn save_image_to_disk(id: &str, filename: &str, data: &[u8]) -> Result<String, String> {
    if id.len() < 10 {
        return Err("Invalid id format".to_string());
    }
    let date = &id[..10];
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return Err("Invalid date in id".to_string());
    }

    let img_dir = entries_dir()
        .join(parts[0])
        .join(parts[1])
        .join(id);
    fs::create_dir_all(&img_dir).map_err(|e| e.to_string())?;

    let path = img_dir.join(filename);
    fs::write(&path, data).map_err(|e| e.to_string())?;

    Ok(format!("./{}/{}", id, filename))
}

pub fn delete_entry_from_disk(id: &str) -> Result<(), String> {
    if id.len() < 10 {
        return Err("Invalid id format".to_string());
    }
    let date = &id[..10];
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return Err("Invalid date in id".to_string());
    }

    let dir = entries_dir().join(parts[0]).join(parts[1]);
    let md_path = dir.join(format!("{}.md", id));
    let img_dir = dir.join(id);

    if md_path.exists() {
        fs::remove_file(&md_path).map_err(|e| e.to_string())?;
    }
    if img_dir.exists() {
        fs::remove_dir_all(&img_dir).map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn parse_frontmatter(content: &str) -> Option<EntryMeta> {
    if !content.starts_with("---") {
        return None;
    }
    let rest = &content[3..];
    let end = rest.find("---")?;
    let yaml = rest[..end].trim();

    let mut id = String::new();
    let mut title = String::new();
    let mut languages: Vec<String> = vec![];
    let mut language = String::new();
    let mut mode = String::new();
    let mut date = String::new();
    let mut date_format = None;
    let mut created_at = None;
    let mut updated_at = None;

    for line in yaml.lines() {
        let line = line.trim();
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            // For created_at/updated_at, the value contains colons (e.g. "2026-02-24T14:30:52")
            // so we need to rejoin everything after the first colon
            let value = if key == "created_at" || key == "updated_at" {
                line.splitn(2, ':').nth(1).unwrap_or("").trim().trim_matches('"')
            } else {
                value.trim().trim_matches('"')
            };
            match key {
                "id" => id = value.to_string(),
                "title" => title = value.to_string(),
                "mode" => mode = value.to_string(),
                "date" => date = value.to_string(),
                "date_format" => date_format = Some(value.to_string()),
                "created_at" => created_at = Some(value.to_string()),
                "updated_at" => updated_at = Some(value.to_string()),
                "language" => language = value.to_string(),
                "languages" => {
                    // Parse YAML inline list: [ja, en] or [ja]
                    let trimmed = value.trim().trim_start_matches('[').trim_end_matches(']');
                    languages = trimmed
                        .split(',')
                        .map(|s| s.trim().trim_matches('"').to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                }
                _ => {}
            }
        }
    }

    // Old format: singular "language" field
    if languages.is_empty() && !language.is_empty() {
        languages = vec![language];
    }

    Some(EntryMeta {
        id,
        date,
        title,
        mode,
        languages,
        date_format,
        created_at,
        updated_at,
    })
}

fn parse_entry(content: &str) -> Result<DiaryEntry, String> {
    let meta = parse_frontmatter(content).ok_or("Failed to parse frontmatter")?;

    // Find content after frontmatter — skip opening "---"
    let rest = &content[3..];
    let fm_end = rest.find("---").ok_or("Invalid frontmatter")? + 3;
    let body = rest[fm_end..].trim();

    let original = extract_section(body, "# Original");
    let mut translations = HashMap::new();

    for lang_code in &meta.languages {
        let header = format!("# {}", lang_code);
        let text = extract_section(body, &header);
        if !text.is_empty() {
            translations.insert(lang_code.clone(), text);
        }
    }

    // Fallback for old format: try section headers like "# Correction" or "# Translation"
    if translations.is_empty() {
        if let Some(first_lang) = meta.languages.first() {
            let fallback_header = if meta.mode == "correction" {
                "# Correction"
            } else {
                "# Translation"
            };
            let text = extract_section(body, fallback_header);
            if !text.is_empty() {
                translations.insert(first_lang.clone(), text);
            }
        }
    }

    Ok(DiaryEntry {
        meta,
        original,
        translations,
    })
}

/// Extract text under a section header up to the next `# ` heading or end of body.
fn extract_section(body: &str, header: &str) -> String {
    let Some(start_idx) = body.find(header) else {
        return String::new();
    };

    let after_header = &body[start_idx + header.len()..];
    // Find the next heading that starts with "# " (at start of line)
    // We look for "\n# " to detect next section
    let section_text = if let Some(next_heading) = find_next_heading(after_header) {
        &after_header[..next_heading]
    } else {
        after_header
    };

    section_text.trim().to_string()
}

/// Find the index of the next `# ` heading in text (must be at start of a line).
fn find_next_heading(text: &str) -> Option<usize> {
    let mut search_from = 0;
    while let Some(newline_pos) = text[search_from..].find('\n') {
        let abs_pos = search_from + newline_pos;
        let rest = &text[abs_pos + 1..]; // content after the newline
        if rest.starts_with("# ") {
            return Some(abs_pos + 1);
        }
        search_from = abs_pos + 1;
        if search_from >= text.len() {
            break;
        }
    }
    None
}

fn serialize_entry(entry: &DiaryEntry) -> String {
    let date_format_line = entry
        .meta
        .date_format
        .as_ref()
        .map(|f| format!("date_format: \"{}\"\n", f))
        .unwrap_or_default();

    let created_at_line = entry
        .meta
        .created_at
        .as_ref()
        .map(|t| format!("created_at: {}\n", t))
        .unwrap_or_default();

    let updated_at_line = entry
        .meta
        .updated_at
        .as_ref()
        .map(|t| format!("updated_at: {}\n", t))
        .unwrap_or_default();

    let languages_str = format!(
        "[{}]",
        entry.meta.languages.join(", ")
    );

    let mut output = format!(
        "---\nid: {}\ntitle: {}\ndate: {}\nmode: {}\nlanguages: {}\n{}{}{}---\n\n# Original\n\n{}\n",
        entry.meta.id,
        entry.meta.title,
        entry.meta.date,
        entry.meta.mode,
        languages_str,
        date_format_line,
        created_at_line,
        updated_at_line,
        entry.original,
    );

    for lang_code in &entry.meta.languages {
        let text = entry.translations.get(lang_code).map(|s| s.as_str()).unwrap_or("");
        output.push_str(&format!("\n# {}\n\n{}\n", lang_code, text));
    }

    output
}
