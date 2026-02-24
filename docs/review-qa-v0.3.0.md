# Diary Sensei v0.3.0 品質審查報告（QA 視角）

> 審查日期：2026-02-24
> 審查角色：QA 工程師

---

## 1. Bug 報告（確定的問題）

### BUG-01: Frontmatter 序列化時 title 沒有 YAML 引號保護 [Critical]

**位置**: `src-tauri/src/storage.rs` serialize_entry

`serialize_entry` 中 title 直接寫入 frontmatter：
```rust
format!("...\ntitle: {}\n...", entry.meta.title)
```

如果 title 包含 YAML 敏感字元如 `#`（開頭）、`---`（整行是分隔符）、或前導特殊字元 `[`、`{`，會破壞後續 frontmatter 解析。

**影響**：日記資料遺失或損壞
**建議**：title 加上引號保護或使用 `serde_yaml` 序列化

### BUG-02: Frontmatter `---` 分隔符匹配過於寬鬆 [Major]

**位置**: `src-tauri/src/storage.rs` 第 243-248 行

```rust
let rest = &content[3..];
let end = rest.find("---")?;
```

`find("---")` 匹配任何位置的三個連續破折號，包括 value 中恰好包含 `---` 的情況。例如 title 值含 `my---test`，會提前終止解析。

**建議**：改為匹配行首的 `\n---\n` 或 `\n---`

### BUG-03: Correction 模式列印時 language fallback 缺失 [Major]

**位置**: `src/App.svelte` 第 341 行 print-view

```svelte
{#if translationsVal[selectedLangsVal[0]]}
```

如果 `selectedLangsVal` 是空陣列，`selectedLangsVal[0]` 是 `undefined`，`translationsVal[undefined]` 永遠是 `undefined`。Corrected 版本不會被列印出來。

但 `handleSubmit` 中有 fallback：`selectedLangsVal[0] || configVal.default_language`

**建議**：print-view 也加入相同 fallback

### BUG-04: Save 不檢查日期是否已設定 [Major]

**位置**: `src/App.svelte` handleSave

`handleSave` 沒有檢查 `dateVal` 是否有值。如果手動清空日期欄位：
1. `generate_entry_id("")` 產生格式錯誤的 ID
2. 路徑解析會失敗回傳 `"Invalid date format"`
3. 不完整日期字串（正在輸入中）也會出錯

**建議**：Save 前驗證 dateVal 格式

### BUG-05: 搜尋清空不會停止進行中的 debounce [Minor]

**位置**: `src/lib/EntryList.svelte` clearSearch

```typescript
function clearSearch() {
    searchQuery.set('');
    searchResults.set(null);
}
```

沒有 `clearTimeout(searchTimeout)`，debounce 中的搜尋仍會觸發，清空後突然出現搜尋結果。

### BUG-06: 多語翻譯串行處理，效能差且容錯差 [Minor]

**位置**: `src-tauri/src/commands.rs` translate_text

for 迴圈依序呼叫每個語言翻譯。3 語言 x 5-10 秒 = 15-30 秒等待。且一個失敗（`?` early return）會丟棄已成功的翻譯結果。

**建議**：改用 `futures::join_all` 平行處理

### BUG-07: API Key 明文儲存 [Minor]

**位置**: `src-tauri/src/config.rs`

API Key 以明文存在 `~/Documents/diary-language/config.json`。

**建議**：考慮使用平台 keychain（tauri-plugin-keychain）

### BUG-08: `entries_dir` 的 `~` 替換不可靠 [Minor]

**位置**: `src-tauri/src/storage.rs` 第 41 行

`replace('~', ...)` 會替換所有出現的 `~`，不只路徑開頭。且 Windows 上 `~` 不是標準路徑。

**建議**：改用 `strip_prefix("~")` 或只替換開頭

---

## 2. 測試案例清單

### 2.1 Correction 模式

| 編號 | 測試案例 | 預期結果 | 類型 |
|------|---------|---------|------|
| C-01 | 輸入正確的日文文字，提交 | 收到微小修正或認可 | Happy path |
| C-02 | 空白內容點 Submit | 按鈕不觸發（被 `!editorVal.trim()` 擋住） | Edge case |
| C-03 | 超長文字（> 10000 字元） | 正常回應或合理錯誤（注意 max_tokens: 4096） | Edge case |
| C-04 | 純 emoji（🎉🎊🎈） | 回傳原文或合理回應 | Edge case |
| C-05 | 中日混合文字 | 只修正日文部分 | Happy path |
| C-06 | 快速連點 Submit 兩次 | 第一次開始後按鈕 disabled | Edge case |
| C-07 | 點 Accept Correction | 編輯器內容替換為修正版 | Happy path |
| C-08 | 切換語言後提交 | 使用新語言 | Happy path |
| C-09 | 只有換行/空白的文字 | 被 trim 擋住 | Edge case |

### 2.2 Translation 模式

| 編號 | 測試案例 | 預期結果 | 類型 |
|------|---------|---------|------|
| T-01 | 中文翻譯成日文 | 正確翻譯 | Happy path |
| T-02 | 多目標語言（ja + en + it） | 三種都顯示 | Happy path |
| T-03 | 翻譯進行中切換模式 | 結果正確對應 | Edge case |
| T-04 | 語言清單為空 | 不 crash | Edge case |
| T-05 | 翻譯結果點「複製」 | 複製到剪貼簿 | Happy path |

### 2.3 右鍵即時翻譯

| 編號 | 測試案例 | 預期結果 | 類型 |
|------|---------|---------|------|
| Q-01 | 選取文字右鍵選語言 | 顯示翻譯泡泡 | Happy path |
| Q-02 | 沒有選取直接右鍵 | 顯示瀏覽器預設 menu | Happy path |
| Q-03 | 選取空白/換行右鍵 | 被 trim 擋住 | Edge case |
| Q-04 | 視窗右下角右鍵 | menu 不應溢出視窗（**目前未處理**） | Edge case |
| Q-05 | 泡泡顯示中點其他位置 | 泡泡關閉 | Happy path |
| Q-06 | 泡泡中點「複製」 | 翻譯複製到剪貼簿 | Happy path |
| Q-07 | AI 翻譯失敗 | 泡泡顯示 Error | Error path |

### 2.4 儲存/讀取

| 編號 | 測試案例 | 預期結果 | 類型 |
|------|---------|---------|------|
| S-01 | 寫日記點 Save | 檔案建立，清單更新 | Happy path |
| S-02 | 儲存後選取同一筆 | 完整還原：標題、日期、內容、模式、翻譯 | Happy path |
| S-03 | 修改既有日記儲存 | `updated_at` 更新，`created_at` 不變 | Happy path |
| S-04 | 標題含 `#` 字元 | 儲存/讀取正確（**可能有問題**） | Edge case |
| S-05 | 標題含 `---` | 儲存/讀取正確（BUG-02 相關） | Edge case |
| S-06 | 標題為空 | 自動使用 `{date} diary` | Edge case |
| S-07 | 內文含 `# Original` 文字 | 不被誤認為 section header | Edge case |
| S-08 | 連續快速點 Save | 不產生重複檔案 | Edge case |
| S-09 | 日期欄位為空時 Save | 應顯示錯誤（**目前可能 crash**） | Edge case |

### 2.5 搜尋

| 編號 | 測試案例 | 預期結果 | 類型 |
|------|---------|---------|------|
| SR-01 | 輸入關鍵字搜尋 | 顯示匹配結果 | Happy path |
| SR-02 | 搜尋空字串 | 還原為月份清單 | Happy path |
| SR-03 | 特殊字元（`#`、`---`、`[`） | 不 crash | Edge case |
| SR-04 | 搜尋 emoji | 正確比對 | Edge case |
| SR-05 | 1000+ 筆日記搜尋 | < 2 秒 | Performance |
| SR-06 | 搜尋後點選結果 | 正確載入 | Happy path |
| SR-07 | 快速輸入（連續按鍵） | debounce 正常 | Edge case |

### 2.6 日曆/日期

| 編號 | 測試案例 | 預期結果 | 類型 |
|------|---------|---------|------|
| D-01 | 點選日曆日期 | 選中，清空編輯器 | Happy path |
| D-02 | 切換上/下月 | 日曆和清單更新 | Happy path |
| D-03 | 2026/1 切到 2025/12 | 跨年正確 | Edge case |
| D-04 | date input 手動改日期 | 日曆同步跳到對應月 | Happy path |
| D-05 | 選擇未來日期 | 允許 | Happy path |
| D-06 | 有日記日期顯示圓點 | 圓點正確 | Happy path |

### 2.7 列印/PDF

| 編號 | 測試案例 | 預期結果 | 類型 |
|------|---------|---------|------|
| P-01 | Correction 有結果時列印 | Original + Corrected | Happy path |
| P-02 | Translation 有結果時列印 | Original + 各語言翻譯 | Happy path |
| P-03 | 沒有翻譯結果時列印 | 只顯示 Original | Edge case |
| P-04 | Correction 但語言未選時列印 | 不顯示 Corrected（BUG-03） | Edge case |

### 2.8 設定

| 編號 | 測試案例 | 預期結果 | 類型 |
|------|---------|---------|------|
| ST-01 | 切換 AI Provider 為 Claude | 顯示 API Key 欄位 | Happy path |
| ST-02 | 儲存設定後重開 | 設定持久化 | Happy path |
| ST-03 | 清空 API Key 後用 Claude 提交 | 顯示 "API key not configured" | Error path |
| ST-04 | Ollama 未啟動時提交 | 顯示 "Cannot connect" | Error path |
| ST-05 | 修改 entries_dir 後儲存 | 使用新路徑 | Happy path |

---

## 3. 風險評估（需實測確認）

### R-01: 大量日記搜尋效能 [中風險]

`search_entries` 走遍所有年/月目錄並 `read_to_string` 每個 .md 檔案。無快取、無索引。數百筆以上可能延遲明顯。加上 debounce 只有 300ms，連續打字可能觸發多次掃描。

### R-02: 並發檔案存取無鎖保護 [中風險]

所有儲存操作直接 `fs::write` / `fs::remove_file`，沒有 file lock。快速連點 Save 可能：
- 兩個 async task 同時寫同一檔案
- Save 進行中 `list_entries` 讀到半成品
- 同一秒內產生相同 ID（`generate_entry_id` 精確到秒）

### R-03: `extract_section` header 匹配可能誤判 [中風險]

`body.find(header)` 做字串搜尋。如果日記原文包含 `# Original` 或 `# ja` 文字（例如寫 Markdown 筆記），會被誤認為 section header。

### R-04: Context menu / Bubble 溢出視窗 [低風險]

使用 `position: fixed` 直接用滑鼠座標定位，沒有邊界偵測。視窗邊緣右鍵時 menu 可能被裁切。

### R-05: CSP 與自訂 Ollama URL 不同步 [低風險]

CSP `connect-src` 寫死 `http://localhost:11434`。如果用戶改成 LAN 其他機器的 URL，連線會被 CSP 擋住。

### R-06: Prompt injection [低風險]

日記內容直接作為 user message 發給 AI。目前低風險因為：
- 本地桌面應用，用戶操作自己的內容
- AI 輸出只顯示為文字，不會被執行

但即時翻譯的短文字片段較容易被 AI 誤解為指令（已有 wrapping 處理）。

### R-07: Svelte 5 legacy/runes 混用 [中風險]

Settings.svelte 使用 `$state`/`$effect`（runes），其他元件用 legacy `subscribe`。混用增加維護複雜度，未來升級可能產生不一致。

### R-08: 跨平台路徑問題 [中風險]

`replace('~', ...)` 是 Unix-style。Windows 上 `~` 不是標準路徑。`save_image_to_disk` 回傳的路徑使用 `/` 作為分隔符，Windows 上可能出問題。

---

## 4. 改善建議

### 4.1 高優先

| 項目 | 說明 |
|------|------|
| **Frontmatter title 引號保護** | 序列化時加引號或改用 serde_yaml |
| **Frontmatter `---` 行首匹配** | 改為搜尋 `\n---` 後跟換行或檔尾 |
| **Save 前驗證** | 檢查 dateVal、selectedLangs |
| **列印 language fallback** | print-view 加入與 handleSubmit 相同的 fallback |

### 4.2 中優先

| 項目 | 說明 |
|------|------|
| **多語翻譯平行處理** | `futures::join_all` 降低等待時間 |
| **搜尋清空時清 debounce** | 加 `clearTimeout(searchTimeout)` |
| **搜尋結果數量上限** | 避免大量日記的效能問題 |
| **CSP 動態調整** | Rust 端 proxy 或動態更新 CSP |

### 4.3 低優先

| 項目 | 說明 |
|------|------|
| Context menu 邊界偵測 | 確保不溢出視窗 |
| API Key 加密儲存 | 使用平台 keychain |
| 統一 Svelte 風格 | 全部 runes 或全部 legacy |
| 增加自動化測試 | 至少為 frontmatter 解析加 Rust 單元測試 |
| Save 按鈕防重複點擊 | 加入 processing 狀態 |

---

## 5. 測試覆蓋率觀察

專案目前 **零測試覆蓋率**（前端無 `.test.ts`，Rust 無 `#[cfg(test)]`）。

**最需要優先補測試的函式**：

1. `parse_frontmatter` — 各種 edge case（空標題、含冒號、YAML 特殊字元、`---` 在 value 中）
2. `serialize_entry` + `parse_entry` round-trip — 序列化後再解析應得相同結果
3. `extract_section` — 多 section、section 不存在、用戶內容含 `# ` 開頭文字
4. `generate_entry_id` — 格式正確性
