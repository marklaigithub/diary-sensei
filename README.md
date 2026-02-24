# Diary Sensei

[English](README.en.md) | [日本語](README.ja.md)

用外語寫日記，AI 幫你修正文法、翻譯對照。隱私優先，預設使用本機 Ollama 運行，資料不離開你的電腦。

## 功能特色

- **文法修正**：用外語書寫，AI 標示錯誤並顯示修正對照（diff 格式）
- **翻譯模式**：用中文書寫，AI 翻譯成目標語言並並排顯示
- **多語言同時翻譯**：一次翻譯成日文 + 英文等多種語言
- **每日多篇**：一天可新增多篇日記
- **Emoji 與圖片**：支援在日記中插入表情符號與圖片
- **PDF 匯出**：將日記匯出為 PDF
- **三種主題**：warm-light（預設）、cool-light、dark
- **本機儲存**：日記以 Markdown + YAML frontmatter 格式儲存於本機
- **隱私優先**：預設使用 Ollama 本機 AI，不呼叫外部 API

## 技術架構

| 層級 | 技術 |
|------|------|
| 前端 | Svelte 5 + TypeScript |
| 後端 | Rust（Tauri 2.x） |
| AI（本機） | Ollama（Gemma 2 9B，預設） |
| AI（雲端） | Claude API（選用） |
| 儲存格式 | Markdown + YAML frontmatter |

## 環境需求

- [Node.js](https://nodejs.org/) 18 以上
- [Rust](https://www.rust-lang.org/tools/install)（含 Cargo）
- [Ollama](https://ollama.com/)（選用，使用本機 AI 時需要）

安裝 Ollama 後，下載 Gemma 2 模型：

```bash
ollama pull gemma2:9b
```

## 安裝與開發

```bash
# 安裝相依套件
npm install

# 啟動開發模式
npm run tauri dev

# 建置正式版本
npm run tauri build
```

## 使用方式

1. 啟動應用程式後，選擇今天的日期或新增日記
2. 選擇模式：**文法修正**（外語書寫）或**翻譯**（中文書寫）
3. 輸入內容後，點擊送出，AI 會回傳修正或翻譯結果
4. 日記自動儲存於本機 Markdown 檔案

**AI 設定**：預設使用 Ollama 本機模型。若需要使用 Claude API，在設定頁面填入 API 金鑰即可切換。

## 授權

[MIT License](LICENSE)
