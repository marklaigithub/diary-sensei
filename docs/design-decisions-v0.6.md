# diary-sensei 設計決策 — v0.6 方向

日期：2026-02-25
來源：AI 委員會 QA 實驗（10 個 AI 來源 × 6 角色）+ 人裁決

## 裁決結果

### D1：Translation mode 可獨立存檔

**決策**：使用者在 Translation mode 能直接存檔，不需切回 Writing mode。

**現況問題**：
- handleSave 在 translation mode 用 `savedWritingContent`（記憶體暫存，可能過期）
- mode 固定存為 `'correction'`，不反映實際模式
- 使用者必須切回 Writing mode 才能存檔 → 不直覺

**期望行為**：
- Translation mode 下按存檔 → 存原文 + 翻譯結果
- 不再需要 `savedWritingContent` 暫存機制（或至少不依賴它做存檔）
- 存檔時 mode 欄位反映實際使用的模式

**影響範圍**：
- `App.svelte`: handleSave、handleModeSwitch
- `storage.rs`: save_entry 可能需支援 translation mode metadata
- `EntryList.svelte`: 載入時可能需辨識不同 mode 的 entry

---

### D2：隱私保護以法規精神為導向

**決策**：即使是個人專案，隱私保護以「使用者有權知道資料去哪」為原則。

**現況問題**：
- 切換 AI provider（Ollama → Claude）無警告，資料靜默送上雲端
- API key 明文存在 config.json
- CSP 不分模式都允許外部連線

**期望行為**：
- 切換到 Claude 時彈出確認對話框：「你的日記內容將傳送到 Anthropic 伺服器」
- 主 UI 明顯顯示當前 AI provider（不只在設定頁）
- API key 改用 OS keychain 或至少加密儲存
- Ollama 模式下收緊 CSP，不允許外部 API 連線

**不做的事**（個人專案階段）：
- 完整 GDPR 合規文件
- 第三方安全稽核
- 資料加密 at rest（Ollama 模式下資料不離開本機，風險低）

**影響範圍**：
- `Settings.svelte`: provider 切換確認 dialog
- `App.svelte` 或頂部 UI: provider 狀態指示器
- `config.rs`: API key 加密/keychain 整合
- `tauri.conf.json`: 動態 CSP 或拆分設定

---

### D3：矯正強度分級

**決策**：新增矯正強度設定（提示/標準/完整），讓使用者控制 AI 介入程度。

**現況問題**：
- AI 全力矯正所有錯誤，沒有強度選項
- 初學者和進階者收到相同的矯正方式
- 可能造成依賴而非學習

**期望行為**：

| 強度 | AI 行為 | 適合誰 |
|------|---------|--------|
| **提示 (Hints)** | 標記錯誤位置，不直接修改，只說明哪裡有問題 | 進階學習者、想自己改的人 |
| **標準 (Standard)** | 修正 + 解釋每個修改（現有行為） | 多數使用者 |
| **完整 (Full)** | 修正文法 + 語氣 + 用字 + 自然度 | 初學者、想看「完美版」的人 |

**影響範圍**：
- `config.rs`: 新增 `correction_intensity` 欄位（"hints" / "standard" / "full"）
- `claude.rs`: 根據強度調整 correction_prompt
- `Settings.svelte`: 新增強度選擇 UI
- 可選：`Correction.svelte` 顯示當前強度

---

## 來自委員會共識的設計前提

以下不是分歧，而是 10 個 AI 來源中 5+ 個都同意的「app 該有但沒有的行為」：

### P1：模式切換要有視覺回饋

切換 Writing ↔ Translation 時，顯示 toast 或 banner 說明「日記內容已保存，這是翻譯草稿區」。
使用者不該因為編輯器清空而恐慌。

### P2：Undo 機制需改善

5 秒 undo 倒數太短且容易錯過。改為：
- 至少 10 秒，或改為手動關閉
- 同時提供 Cmd+Z 復原路徑

### P3：需要首次使用引導

新使用者打開 app 看到空的三欄介面會不知道要幹嘛。加入：
- 首次啟動的語言 / AI backend 設定精靈
- 或至少一個「開始使用」的提示

### P4：語言應綁定到日記條目

開啟舊的日文日記時，矯正語言應自動切到日文，不該用全域設定的韓文去矯正。

### P5：搜尋需要索引

300+ 條目時全目錄掃描太慢。需要建立某種索引機制。

---

## 實作優先順序建議

基於影響範圍和風險排序：

| 優先級 | 項目 | 理由 |
|--------|------|------|
| 1 | P1 模式切換回饋 | 改動小、效果大、所有來源都提到 |
| 2 | D1 Translation mode 存檔 | 核心 UX 問題，且修好後 TS-03（race condition）自然解決 |
| 3 | D3 矯正強度 | 獨立功能，不影響現有邏輯 |
| 4 | P2 Undo 改善 | 改動小 |
| 5 | P4 語言綁定 | 改動中等 |
| 6 | D2 隱私保護 | provider 確認 dialog 簡單，keychain 較複雜 |
| 7 | P3 首次引導 | 可以簡單做 |
| 8 | P5 搜尋索引 | 可延後，目前條目數不多 |
