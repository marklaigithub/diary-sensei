#!/bin/bash
# Committee QA Experiment - Ollama Side
# Runs 4 models × 4 personas = 16 queries (sequential per model, models can overlap)

OUTDIR="/Users/marklai/Code/diary-sensei/experiments/responses"
mkdir -p "$OUTDIR"

# App description (shared context)
read -r -d '' APP_DESC << 'APPDESC'
diary-sensei is a desktop diary application for language learners.
You write diary entries in a foreign language you're learning, and AI helps you.

Two modes:
1. Writing mode (Correction): Write in a foreign language, AI corrects your grammar and explains what was wrong
2. Translation mode: Write in any language, AI translates to multiple languages at once

How it looks:
- 3-column layout: Left sidebar (calendar + entry list) | Center (text editor) | Right (AI results)
- Calendar shows dots on days that have entries
- Click a date to see entries for that day
- Right-click any text in the editor for quick translate popup

Key features:
- AI backends: Ollama (runs locally, private) or Claude API (cloud)
- Entries saved as Markdown files in a folder on your computer
- 5 UI languages: English, Traditional Chinese, Japanese, Korean, Italian
- Search across all entries
- 3 themes: warm light, cool light, dark
- PDF export / print
- Each language can have its own date format
- Undo button appears for 5 seconds after applying a correction
- Unsaved changes warning when switching entries
- Switching to Translation mode gives you a separate scratch pad (diary content preserved)
- When you save in Translation mode, it saves your original diary content, not the scratch pad
APPDESC

# Persona definitions
declare -A PERSONAS
PERSONAS[first-time-user]="You are someone who just downloaded this app for the first time. You want to learn Japanese. You are not very tech-savvy. You opened the app and are trying to figure out how to use it."
PERSONAS[heavy-user]="You have been using this app every day for 6 months. You have over 300 diary entries. You are learning Japanese and Korean simultaneously. You rely on this app as your primary language learning journal."
PERSONAS[privacy-paranoid]="You chose this app specifically because it supports local AI (Ollama). You do NOT want any of your diary content sent to the cloud. You are suspicious of any network activity. Privacy is your top priority."
PERSONAS[clumsy-user]="You are a careless user. You click things randomly, you don't read warnings, you sometimes close the app without saving, you paste very long text, you switch between features quickly without waiting for things to finish."

# Core questions
read -r -d '' QUESTIONS << 'QS'
Answer these questions from your perspective:

1. What would confuse or frustrate you about this app?
2. What would you try to do that might not work or behave unexpectedly?
3. What's missing that you'd expect to be there?
4. If something went wrong (crash, lost data, wrong AI result), what would you do?
5. What would make you stop using this app?
6. Describe a realistic scenario where you use this app and something goes wrong.

Be specific. Give concrete examples from real scenarios, not abstract principles.
Keep your answer under 500 words.
QS

# Models to test
MODELS=("gemma2:9b" "llama3.2:3b" "phi4-mini" "mistral:7b")

run_query() {
    local model="$1"
    local persona_key="$2"
    local persona_desc="$3"
    local safe_model=$(echo "$model" | tr ':' '-')
    local outfile="$OUTDIR/ollama_${safe_model}_${persona_key}.md"

    echo "[$(date +%H:%M:%S)] Running: $model as $persona_key..."

    local prompt="$persona_desc

Here is the app you are evaluating:

$APP_DESC

$QUESTIONS"

    # Build JSON payload (escape for JSON)
    local json_payload
    json_payload=$(jq -n \
        --arg model "$model" \
        --arg prompt "$prompt" \
        '{model: $model, messages: [{role: "user", content: $prompt}], stream: false, options: {temperature: 0.8, num_predict: 1024}}')

    local start_time=$(date +%s)
    local response
    response=$(curl -s --max-time 180 http://localhost:11434/api/chat -d "$json_payload" 2>&1)
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))

    # Extract the message content
    local content
    content=$(echo "$response" | jq -r '.message.content // "ERROR: No response"' 2>/dev/null)

    if [ -z "$content" ] || [ "$content" = "null" ] || [ "$content" = "ERROR: No response" ]; then
        content="ERROR: Failed to get response. Raw: $(echo "$response" | head -c 500)"
    fi

    # Write output
    cat > "$outfile" << EOF
# $model as $persona_key
- Duration: ${duration}s
- Temperature: 0.8

## Persona
$persona_desc

## Response
$content
EOF

    echo "[$(date +%H:%M:%S)] Done: $model as $persona_key (${duration}s)"
}

echo "=== Committee QA Experiment - Ollama Side ==="
echo "Started at: $(date)"
echo ""

# Run all combinations sequentially (Ollama can only run one model at a time efficiently)
for persona_key in "${!PERSONAS[@]}"; do
    for model in "${MODELS[@]}"; do
        run_query "$model" "$persona_key" "${PERSONAS[$persona_key]}"
        echo ""
    done
done

echo ""
echo "=== All queries complete ==="
echo "Results in: $OUTDIR/"
echo "Finished at: $(date)"
