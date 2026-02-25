# Committee QA Experiment - diary-sensei
# Date: 2026-02-25

## App Description (for Ollama models - user perspective only)

diary-sensei is a desktop diary application for language learners.
You write diary entries in a foreign language you're learning, and AI helps you.

### Two modes:
1. **Writing mode (Correction)**: Write in a foreign language → AI corrects your grammar and explains what was wrong
2. **Translation mode**: Write in any language → AI translates to multiple languages at once

### How it looks:
- 3-column layout: Left sidebar (calendar + entry list) | Center (text editor) | Right (AI results)
- Calendar shows dots on days that have entries
- Click a date → see entries for that day
- Right-click any text in the editor → quick translate popup

### Key features:
- AI backends: Ollama (runs locally, private) or Claude API (cloud)
- Entries saved as Markdown files in a folder on your computer
- 5 UI languages: English, 繁體中文, 日本語, 한국어, Italiano
- Search across all entries
- 3 themes: warm light, cool light, dark
- PDF export / print
- Each language can have its own date format
- "Undo" button appears for 5 seconds after applying a correction
- Unsaved changes warning when switching entries
- Delete entries (with confirmation)

### What's NOT obvious from the UI:
- Switching to Translation mode gives you a separate scratch pad (your diary content is preserved)
- When you save in Translation mode, it saves your original diary content, not the scratch pad
- Entries are stored as YYYY-MM-DD_HHMMSS.md files organized by year/month folders

---

## Core Questions (same for all roles)

1. What would confuse or frustrate you about this app?
2. What would you try to do that might not work or behave unexpectedly?
3. What's missing that you'd expect to be there?
4. If something went wrong (crash, lost data, wrong AI result), what would you do?
5. What would make you stop using this app?
6. Describe a realistic scenario where you'd use this app and something goes wrong.

Be specific. Give concrete examples. Think about real scenarios, not abstract principles.
