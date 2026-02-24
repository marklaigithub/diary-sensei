# Diary Sensei

[繁體中文](README.md) | [English](README.en.md)

外国語で日記を書き、AIが文法を添削・翻訳します。プライバシー優先設計で、デフォルトはローカルのOllamaを使用——データは外部に送信されません。

## 機能

- **文法添削**：外国語で書いた文章をAIが添削し、差分形式で修正箇所を表示
- **翻訳モード**：中国語で書いた内容をAIが目標言語に翻訳し、並べて表示
- **複数言語への同時翻訳**：日本語・英語など複数言語に一括翻訳
- **1日複数エントリ**：1日に何件でも日記を追加可能
- **絵文字・画像サポート**：日記に絵文字や画像を挿入可能
- **PDFエクスポート**：日記をPDFとして書き出し
- **3種類のテーマ**：warm-light（デフォルト）、cool-light、dark
- **ローカル保存**：Markdown + YAMLフロントマター形式で端末に保存
- **プライバシー優先**：デフォルトでOllamaを使用し、外部APIへのデータ送信なし

## 技術スタック

| レイヤー | 技術 |
|---------|------|
| フロントエンド | Svelte 5 + TypeScript |
| バックエンド | Rust（Tauri 2.x） |
| AI（ローカル） | Ollama（Gemma 2 9B、デフォルト） |
| AI（クラウド） | Claude API（オプション） |
| ストレージ | Markdown + YAMLフロントマター |

## 必要な環境

- [Node.js](https://nodejs.org/) 18以上
- [Rust](https://www.rust-lang.org/tools/install)（Cargo含む）
- [Ollama](https://ollama.com/)（オプション、ローカルAI使用時に必要）

Ollamaインストール後、Gemma 2モデルを取得してください：

```bash
ollama pull gemma2:9b
```

## セットアップと開発

```bash
# 依存関係のインストール
npm install

# 開発モードの起動
npm run tauri dev

# 本番ビルド
npm run tauri build
```

## 使い方

1. アプリを起動し、日付を選択するか新しいエントリを作成
2. モードを選択：**文法添削**（外国語で記入）または **翻訳**（中国語で記入）
3. 内容を送信すると、AIが添削または翻訳結果を返す
4. エントリはローカルのMarkdownファイルに自動保存

**AI設定**：デフォルトはOllamaのローカルモデルを使用。Claude APIを利用する場合は、設定画面でAPIキーを入力して切り替えてください。

## ライセンス

[MIT License](LICENSE)
