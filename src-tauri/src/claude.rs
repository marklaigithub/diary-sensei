use reqwest::Client;
use serde::{Deserialize, Serialize};

// --- Shared prompt builders ---

pub fn correction_prompt(language: &str) -> String {
    format!(
        "You are a {} language teacher correcting a student's diary.\n\n\
        RULES:\n\
        1. The diary contains text in MULTIPLE languages.\n\
        2. ONLY correct text written in {} — fix grammar, unnatural expressions, and word choice.\n\
        3. Any text in other languages (English, Chinese, Korean, etc.) MUST be kept EXACTLY as-is, character for character, in the same position.\n\
        4. Do NOT remove, translate, or modify text that is not in {}.\n\
        5. Keep all line breaks, emoji, and image references (![...](path)) unchanged.\n\
        6. Return the COMPLETE diary text with only the {} portions corrected. No explanations.",
        language, language, language, language
    )
}

pub fn translation_prompt(target_language: &str) -> String {
    format!(
        "You are a professional translator. \
        The user's diary entry may contain text in multiple languages (e.g., Chinese, Japanese, English, etc.). \
        Translate ALL content into {}, regardless of the original language. \
        Write naturally, as if a native {} speaker wrote the diary. \
        Keep the personal diary tone. Preserve all emoji and image references (![...](path)). \
        Return ONLY the translation, nothing else.",
        target_language, target_language
    )
}

// --- Claude API ---

#[derive(Serialize)]
struct ClaudeMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<ClaudeMessage>,
}

#[derive(Deserialize)]
struct ClaudeContentBlock {
    text: Option<String>,
}

#[derive(Deserialize)]
struct ClaudeResponse {
    content: Vec<ClaudeContentBlock>,
}

pub async fn call_claude(api_key: &str, system: &str, user_text: &str) -> Result<String, String> {
    let client = Client::new();

    let request = ClaudeRequest {
        model: "claude-haiku-4-5-20251001".to_string(),
        max_tokens: 4096,
        system: system.to_string(),
        messages: vec![ClaudeMessage {
            role: "user".to_string(),
            content: user_text.to_string(),
        }],
    };

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("HTTP error: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Claude API error {}: {}", status, body));
    }

    let api_response: ClaudeResponse = response
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    api_response
        .content
        .first()
        .and_then(|block| block.text.clone())
        .ok_or_else(|| "Empty response from Claude".to_string())
}

// --- Ollama API ---

#[derive(Serialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponseMessage {
    content: Option<String>,
}

#[derive(Deserialize)]
struct OllamaResponse {
    message: Option<OllamaResponseMessage>,
    error: Option<String>,
}

pub async fn call_ollama(
    base_url: &str,
    model: &str,
    system: &str,
    user_text: &str,
) -> Result<String, String> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| format!("Client error: {}", e))?;

    let request = OllamaRequest {
        model: model.to_string(),
        messages: vec![
            OllamaMessage {
                role: "system".to_string(),
                content: system.to_string(),
            },
            OllamaMessage {
                role: "user".to_string(),
                content: user_text.to_string(),
            },
        ],
        stream: false,
    };

    let url = format!("{}/api/chat", base_url.trim_end_matches('/'));

    let response = client
        .post(&url)
        .header("content-type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| {
            if e.is_connect() {
                "Cannot connect to Ollama. Make sure it's running: ollama serve".to_string()
            } else if e.is_timeout() {
                "Ollama request timed out. The model might be loading for the first time.".to_string()
            } else {
                format!("HTTP error: {}", e)
            }
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Ollama error {}: {}", status, body));
    }

    let ollama_response: OllamaResponse = response
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    if let Some(err) = ollama_response.error {
        return Err(format!("Ollama error: {}", err));
    }

    ollama_response
        .message
        .and_then(|m| m.content)
        .ok_or_else(|| "Empty response from Ollama".to_string())
}
