# Diary Sensei

[繁體中文](README.md) | [日本語](README.ja.md)

Write diary entries in a foreign language and let AI correct your grammar or translate from Chinese — all with side-by-side diff display. Privacy-first: defaults to local Ollama, no data leaves your machine.

## Features

- **Grammar correction**: Write in a foreign language; AI highlights errors and shows corrections in diff format
- **Translation mode**: Write in Chinese; AI translates to your target languages with side-by-side display
- **Multi-language translation**: Translate to Japanese + English (or more) simultaneously
- **Multiple entries per day**: Add as many entries as you want for a single day
- **Emoji and image support**: Insert emoji and images directly into diary entries
- **PDF export**: Export diary entries as PDF
- **Three themes**: warm-light (default), cool-light, dark
- **Local storage**: Entries saved as Markdown files with YAML frontmatter
- **Privacy-first**: Defaults to local Ollama — no external API calls required

## Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | Svelte 5 + TypeScript |
| Backend | Rust (Tauri 2.x) |
| AI (local) | Ollama (Gemma 2 9B, default) |
| AI (cloud) | Claude API (optional) |
| Storage | Markdown + YAML frontmatter |

## Prerequisites

- [Node.js](https://nodejs.org/) 18 or later
- [Rust](https://www.rust-lang.org/tools/install) (with Cargo)
- [Ollama](https://ollama.com/) (optional, required for local AI)

After installing Ollama, pull the Gemma 2 model:

```bash
ollama pull gemma2:9b
```

## Getting Started

```bash
# Install dependencies
npm install

# Start development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Usage

1. Launch the app and select a date or create a new entry
2. Choose a mode: **Grammar Correction** (write in a foreign language) or **Translation** (write in Chinese)
3. Submit your entry; AI returns corrections or translations
4. Entries are automatically saved as local Markdown files

**AI settings**: Defaults to local Ollama. To use Claude API instead, enter your API key in the settings page.

## License

[MIT License](LICENSE)
