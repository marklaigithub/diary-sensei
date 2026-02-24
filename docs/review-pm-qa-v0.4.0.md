# Diary Sensei v0.4.0 品質審查報告（PM + QA 視角）

> 審查日期：2026-02-24
> 審查角色：PM（產品經理）+ QA（品質工程師）
> 審查範圍：v0.3.0 → v0.4.0 全部變更

---

## 0. v0.4.0 變更總覽

| # | 項目 | 類型 | 狀態 |
|---|------|------|------|
| 1 | 未儲存變更保護（dirty check） | 功能 | ✅ 已實作 |
| 2 | 主題切換持久化 | 修正 | ✅ 已實作 |
| 3 | 刪除日記 UI | 功能 | ✅ 已實作 |
| 4 | 語言列表可編輯 | 功能 | ✅ 已實作 |
| 5 | AI 修正附帶解釋 | 功能 | ✅ 已實作 |
| 6 | Frontmatter title 引號保護 | 修正 | ✅ 已修正 |
| 7 | Frontmatter `---` 分隔符修正 | 修正 | ✅ 已修正 |
| 8 | 列印 language fallback | 修正 | ✅ 已修正 |
| 9 | Save 日期驗證 | 修正 | ✅ 已修正 |
| 10 | 搜尋 debounce 清理 | 修正 | ✅ 已修正 |
| 11 | 多語翻譯平行化 | 修正 | ✅ 已修正 |
| 12 | Rust 單元測試 | 測試 | ✅ 17/17 通過 |

**Build 驗證**：`cargo test` 17/17 通過、`npm run build` 成功

---

## 1. PM 視角：產品評估

### 1.1 P0 需求達成評估

#### Dirty Check（#1）— ✅ 達成

**覆蓋的操作**：
- `handleEntrySelect()` — 選擇其他日記 ✅
- `handleDateSelect()` — 從月曆選日期 ✅
- `handleDateInputChange()` — date input 改日期 ✅
- `handleNewEntry()` — 新增日記 ✅

**未覆蓋的操作**（PM 追加建議）：

| 操作 | 是否保護 | 風險 |
|------|---------|------|
| 關閉視窗（window close） | ❌ | 高 — 最常見的資料遺失場景 |
| 瀏覽器重新整理（dev 環境） | ❌ | 低 — 僅開發時 |
| 切換到 Settings | ❌ | 中 — Settings 覆蓋了編輯器區域 |

**PM-NEW-01**: 視窗關閉時的 dirty check 是最關鍵的保護場景。建議下一版加入 Tauri 的 `window.onCloseRequested` 事件攔截。

**PM-NEW-02**: `isDirty` 在 `onMount` 後 subscribe 到 `editorContent` 和 `entryTitle`，但初始載入時（config load → loadEntries）也會觸發 `isDirty.set(true)`。雖然 `onMount` 末尾有 `isDirty.set(false)` 重置，但若 loadEntries 是非同步完成的，可能出現時序問題。建議觀察實際行為。

#### 主題持久化（#2）— ✅ 達成

實作簡潔，fire-and-forget 模式符合預期。無額外問題。

### 1.2 P1 需求達成評估

#### 刪除日記 UI（#3）— ✅ 達成

- Hover 顯示 ✕ 按鈕，設計內斂不干擾
- Tauri dialog 確認，防誤刪
- 刪除後正確重新載入列表
- 刪除當前編輯中的日記時清空編輯器 ✅

**PM-NEW-03**: 刪除確認框的文字是英文（`Delete "{title}"?`），而 app 本身用戶可能是日文/中文使用者。未來考慮 i18n。

#### 語言列表可編輯（#4）— ✅ 達成

- Inline 編輯 form（不用 modal）✅
- 新增/編輯/刪除功能齊全 ✅
- 所有變更隨 Save Settings 一起存 ✅

**PM-NEW-04**: 刪除語言時沒有確認對話框。如果用戶誤刪 `ja`（主要使用語言），既有日記可能無法正確顯示翻譯結果。建議刪除語言時加入確認，或至少禁止刪除 `default_language` 所指向的語言。

**PM-NEW-05**: 語言 code 沒有唯一性檢查。用戶可以新增兩個 `ja`，造成混亂。

#### AI 修正附帶解釋（#5）— ✅ 達成

- Prompt 改為要求 `[CORRECTED]` + `[EXPLANATION]` 結構化輸出 ✅
- Rust 端解析分離，有 backward-compat fallback ✅
- 前端在 diff 下方顯示 "Why these changes?" 區塊 ✅

**PM 觀察**：這是 v0.4.0 最大的使用者價值提升。從「修正機器」升級為「語言老師」，解釋讓使用者理解為什麼改、怎麼改，學習效果大幅提升。

**PM-NEW-06**: Explanation 目前是純文字顯示。若 AI 回傳的解釋包含 Markdown 格式（如 `**粗體**`、`1. 2. 3.` 列表），可能顯示為原始文字。未來考慮用 Markdown renderer。

### 1.3 PM 整體評分

| 面向 | 評分 | 說明 |
|------|------|------|
| 功能完整度 | 9/10 | 12 項全部交付，缺 window close dirty check |
| 使用者體驗 | 8/10 | 核心流程明顯改善，細節待打磨 |
| 學習價值 | 9/10 | AI 解釋功能是亮點 |
| 穩定性信心 | 8/10 | 有 17 個 Rust 測試，前端無測試 |

---

## 2. QA 視角：Bug 報告

### 2.1 v0.3.0 Bug 修正驗證

| 編號 | 問題 | 修正狀態 | 驗證方式 |
|------|------|---------|---------|
| BUG-01 | Title 引號保護 | ✅ 已修正 | Round-trip 測試 x4（含引號、冒號、`#`、`---`） |
| BUG-02 | `---` 分隔符寬鬆匹配 | ✅ 已修正 | `test_frontmatter_closing_delimiter_not_confused_with_value` |
| BUG-03 | 列印 language fallback | ✅ 已修正 | `selectedLangsVal[0] \|\| configVal?.default_language` |
| BUG-04 | Save 日期驗證 | ✅ 已修正 | regex `/^\d{4}-\d{2}-\d{2}$/` 前端攔截 |
| BUG-05 | 搜尋 debounce 清理 | ✅ 已修正 | `clearTimeout(searchTimeout)` 加入 `clearSearch()` |
| BUG-06 | 翻譯串行處理 | ✅ 已修正 | `futures::future::join_all` 平行化 |
| BUG-07 | API Key 明文 | ⏸ 未處理 | 不在 v0.4.0 範圍 |
| BUG-08 | `~` 替換不可靠 | ⏸ 未處理 | 不在 v0.4.0 範圍 |

### 2.2 v0.4.0 新發現的問題

#### NEW-BUG-01: Dirty flag 在初始載入時被誤設 [Major]

**位置**: `src/App.svelte` 第 54-55 行

```typescript
editorContent.subscribe(() => isDirty.set(true));
entryTitle.subscribe(() => isDirty.set(true));
```

這兩行 subscribe 會在**任何** store 更新時觸發，包括：
- `onMount` 中初始載入時（第 71 行有 `isDirty.set(false)` 補救）
- `handleEntrySelect` 載入日記時（第 97-101 行設定各 store，每次都觸發 isDirty）
- `handleApplyCorrection` 接受修正時

雖然這些 handler 末尾都有 `isDirty.set(false)` 重置，但存在一個問題：**subscribe 的回呼是同步的**，而 `handleEntrySelect` 中 `isDirty.set(false)` 在設定完所有 store 後才執行。這代表在設定 `editorContent` 和 `entryTitle` 的瞬間，isDirty 已被設為 true，然後才被設回 false。

**風險**：目前因為是同步操作，時序上是安全的（dirty → false 在同一個 microtask 內）。但這是一個脆弱的假設，如果未來有 effect 或 derived 依賴 isDirty，可能出現中間狀態問題。

**建議**：改用一個 flag 來暫時停止 dirty tracking：
```typescript
let skipDirtyTracking = false;
editorContent.subscribe(() => { if (!skipDirtyTracking) isDirty.set(true); });
```

#### NEW-BUG-02: handleApplyCorrection 不重置 dirty flag [Minor]

**位置**: `src/App.svelte` 第 235-242 行

```typescript
function handleApplyCorrection() {
    const langKey = selectedLangsVal[0] || configVal.default_language;
    const corrected = translationsVal[langKey];
    if (corrected) {
      editorContent.set(corrected);  // 觸發 isDirty = true
      translations.set({});
    }
    // 沒有 isDirty.set(false)
}
```

Accept Correction 後，編輯器內容被替換為修正版本，`isDirty` 被設為 true。這其實是**合理的**（用戶應該存檔修正版本），但從 UX 角度：用戶如果還沒 Save 就切到別的日記，會被 dirty check 攔住。這**可能是符合預期的行為**，但值得確認。

#### NEW-BUG-03: 刪除語言時 default_language 可能失效 [Major]

**位置**: `src/lib/Settings.svelte` 第 60-62 行

```typescript
function deleteLang(idx: number) {
    languages = languages.filter((_, i) => i !== idx);
}
```

如果用戶刪除了 `default_language` 所指向的語言（如 `ja`），但 `defaultLang` select 的 options 來自 `configVal?.languages`（第 163 行），不是來自本地 `languages` state。這表示：
- options 列表不會即時更新（仍顯示舊的）
- Save 後 default_language 指向一個不存在的語言 code

**建議**：刪除語言後檢查 `defaultLang` 是否仍在列表中，如果不在則自動切換到第一個語言。

#### NEW-BUG-04: Settings 的 Default Language select 不反映即時編輯 [Minor]

**位置**: `src/lib/Settings.svelte` 第 162-166 行

```svelte
<select class="setting-input" bind:value={defaultLang}>
    {#each configVal?.languages || [] as lang}
        <option value={lang.code}>{lang.name} ({lang.code})</option>
    {/each}
</select>
```

Options 來自 `configVal?.languages`（儲存的設定），而非本地的 `languages` state。如果用戶新增了一個語言 `ko`，在 Save 之前，`ko` 不會出現在 Default Language 的下拉選單中。

**建議**：改為 `{#each languages as lang}`。

#### NEW-BUG-05: Explanation 在切換日記後不清空 [Minor]

**位置**: `src/App.svelte`

`handleEntrySelect`（第 89-106 行）、`handleNewEntry`（第 209-217 行）、`handleDateSelect`（第 108-118 行）載入/切換日記時，都沒有重置 `explanation` store。如果用戶在日記 A 做了 correction 看到 explanation，然後切到日記 B，explanation 區塊仍然顯示日記 A 的解釋。

**建議**：在這三個 handler 中加入 `explanation.set(null)`。

#### NEW-BUG-06: Correction panel 的語言取值不穩定 [Minor]

**位置**: `src/lib/Correction.svelte` 第 17-20 行

```typescript
translations.subscribe(v => {
    const lang = selectedLangs[0];
    corrected = lang ? (v[lang] || '') : '';
});
```

`selectedLangs` 在 translations 變更時讀取，但 `selectedLangs` 本身可能還沒更新。兩個 subscribe 的執行順序取決於 Svelte 的 subscription 順序，而非業務邏輯順序。

**風險**：低。目前 `selectedLangs` 在正常流程中會先被設定（handleEntrySelect 先設 `selectedTargetLanguages`，後設 `translations`），但如果順序改變可能出問題。

#### NEW-BUG-07: 平行翻譯的 Ollama 可能造成資源爭搶 [低風險]

**位置**: `src-tauri/src/commands.rs` translate_text

使用 `join_all` 平行呼叫 Ollama 時，如果是本地 Ollama 且只有一個 GPU，3 個平行請求會排隊或爭搶 VRAM。對於本地模型，效能提升可能不如預期，但至少不會退化。Claude API 場景下確實有顯著加速。

### 2.3 v0.3.0 遺留風險更新

| 風險 | v0.3.0 狀態 | v0.4.0 狀態 | 變化 |
|------|------------|------------|------|
| R-01 搜尋效能 | 中 | 中 | 未改變 |
| R-02 並發檔案存取 | 中 | 中 | 未改變 |
| R-03 extract_section 誤判 | 中 | 中 | 未改變 |
| R-04 Context menu 溢出 | 低 | 低 | 未改變 |
| R-05 CSP 與 Ollama URL | 低 | 低 | 未改變 |
| R-06 Prompt injection | 低 | 低 | 未改變 |
| R-07 Svelte 風格混用 | 中 | 中 | Settings 用 $state，其餘用 legacy subscribe |
| R-08 跨平台路徑 | 中 | 中 | 未改變 |

---

## 3. 測試案例（v0.4.0 新增）

### 3.1 Dirty Check

| 編號 | 測試案例 | 預期結果 | 類型 |
|------|---------|---------|------|
| DC-01 | 編輯後選擇其他日記 | 彈出確認框 | Happy path |
| DC-02 | 編輯後從月曆選日期 | 彈出確認框 | Happy path |
| DC-03 | 編輯後改 date input | 彈出確認框 | Happy path |
| DC-04 | 編輯後點 New | 彈出確認框 | Happy path |
| DC-05 | 編輯後點 Save 再切換 | 不彈確認框 | Happy path |
| DC-06 | 載入日記（不編輯）後切換 | 不彈確認框 | Edge case |
| DC-07 | 確認框選「取消」 | 保留在當前日記 | Happy path |
| DC-08 | 空白編輯器（無修改）切換 | 不彈確認框 | Edge case |
| DC-09 | Accept Correction 後切換 | 彈出確認框（預期行為） | Edge case |
| DC-10 | App 初始載入後立刻切換 | 不應彈確認框（NEW-BUG-01 相關） | Edge case |

### 3.2 刪除日記

| 編號 | 測試案例 | 預期結果 | 類型 |
|------|---------|---------|------|
| DEL-01 | Hover 日記項目 | 顯示 ✕ 刪除按鈕 | Happy path |
| DEL-02 | 點擊 ✕ 按鈕 | 彈出確認框 | Happy path |
| DEL-03 | 確認刪除 | 檔案被刪除，列表更新 | Happy path |
| DEL-04 | 取消刪除 | 不刪除 | Happy path |
| DEL-05 | 刪除正在編輯的日記 | 編輯器清空 | Edge case |
| DEL-06 | 刪除非當前編輯的日記 | 編輯器不受影響 | Edge case |
| DEL-07 | 刪除搜尋結果中的日記 | 結果列表更新（**待確認**：目前 `handleEntryDelete` 呼叫 `loadEntries` 而非更新搜尋結果） | Edge case |

### 3.3 語言列表編輯

| 編號 | 測試案例 | 預期結果 | 類型 |
|------|---------|---------|------|
| LG-01 | 點 + Add Language | 顯示 inline form | Happy path |
| LG-02 | 填入 code + name，點 ✓ | 新語言加入列表 | Happy path |
| LG-03 | code 或 name 為空，點 ✓ | 不加入（靜默忽略） | Edge case |
| LG-04 | 點 ✕ 取消新增 | form 關閉 | Happy path |
| LG-05 | Hover 現有語言，點 ✎ | 顯示 inline 編輯 form | Happy path |
| LG-06 | 修改後點 ✓ | 更新語言資訊 | Happy path |
| LG-07 | 點 ✕ 刪除語言 | 直接刪除（無確認） | Edge case |
| LG-08 | 刪除 default_language | default_language 可能失效（NEW-BUG-03） | Edge case |
| LG-09 | 新增重複 code 的語言 | 允許（NEW-BUG — 無唯一性檢查） | Edge case |
| LG-10 | 編輯中切換到編輯其他語言 | 未儲存的編輯丟失？ | Edge case |
| LG-11 | Save Settings 後重開 app | 語言列表正確持久化 | Happy path |

### 3.4 AI 修正解釋

| 編號 | 測試案例 | 預期結果 | 類型 |
|------|---------|---------|------|
| EX-01 | 提交需要修正的文字 | Diff + Corrected + Explanation 三區顯示 | Happy path |
| EX-02 | 提交完美文字 | "Perfect!" badge，Explanation 可能顯示 "No corrections needed" | Happy path |
| EX-03 | AI 未回傳 [EXPLANATION] 標記 | 只顯示 corrected，不顯示解釋區 | Fallback |
| EX-04 | AI 未回傳 [CORRECTED] 標記 | 整段視為 corrected text（backward compat） | Fallback |
| EX-05 | 切換到其他日記後回來 | Explanation 被清空（**NEW-BUG-05：目前不會清空**） | Edge case |
| EX-06 | Explanation 含多行 | 正確顯示（white-space: pre-wrap） | Happy path |

### 3.5 Frontmatter Round-trip（自動化測試）

| 編號 | 測試案例 | 測試函式 | 狀態 |
|------|---------|---------|------|
| FT-01 | 正常 frontmatter | `test_parse_frontmatter_normal` | ✅ Pass |
| FT-02 | Title 含冒號 | `test_parse_frontmatter_title_with_colon` | ✅ Pass |
| FT-03 | Title 含 #、--- | `test_parse_frontmatter_title_with_yaml_special_chars` | ✅ Pass |
| FT-04 | Title 含轉義引號 | `test_parse_frontmatter_title_with_escaped_quotes` | ✅ Pass |
| FT-05 | 多語言陣列 | `test_parse_frontmatter_multiple_languages` | ✅ Pass |
| FT-06 | 無 frontmatter | `test_parse_frontmatter_no_frontmatter` | ✅ Pass |
| FT-07 | 空 frontmatter | `test_parse_frontmatter_empty_frontmatter` | ✅ Pass |
| FT-08 | 舊格式 singular language | `test_parse_frontmatter_old_singular_language` | ✅ Pass |
| FT-09 | Value 中含 --- | `test_frontmatter_closing_delimiter_not_confused_with_value` | ✅ Pass |
| FT-10 | Round-trip 基礎 | `test_round_trip_basic` | ✅ Pass |
| FT-11 | Round-trip 特殊標題 | `test_round_trip_special_title` | ✅ Pass |
| FT-12 | Round-trip 引號標題 | `test_round_trip_title_with_quotes` | ✅ Pass |
| FT-13 | Round-trip 多語言 | `test_round_trip_multi_language` | ✅ Pass |
| FT-14 | Section 擷取正常 | `test_extract_section_normal` | ✅ Pass |
| FT-15 | Section 不存在 | `test_extract_section_not_found` | ✅ Pass |
| FT-16 | 內文含 # 不誤判 | `test_extract_section_body_contains_hash_in_text` | ✅ Pass |
| FT-17 | Entry ID 格式 | `test_generate_entry_id_format` | ✅ Pass |

---

## 4. 改善建議（v0.5.0 候選）

### 4.1 高優先

| 項目 | 來源 | 說明 |
|------|------|------|
| **Window close dirty check** | PM-NEW-01 | `window.onCloseRequested` 事件攔截 |
| **Explanation 切換日記時清空** | NEW-BUG-05 | 三個 handler 加 `explanation.set(null)` |
| **Default Language 下拉即時反映** | NEW-BUG-04 | 改用本地 `languages` state |
| **刪除語言防護** | NEW-BUG-03 | 禁止刪除 default_language、加確認 |

### 4.2 中優先

| 項目 | 來源 | 說明 |
|------|------|------|
| **Dirty tracking 可暫停機制** | NEW-BUG-01 | 用 flag 控制 subscribe 觸發 |
| **語言 code 唯一性檢查** | PM-NEW-05 | 新增/編輯時檢查 code 不重複 |
| **搜尋結果中刪除的即時更新** | DEL-07 | 刪除後重新搜尋或從結果中移除 |
| **Explanation Markdown 渲染** | PM-NEW-06 | 改用 Markdown renderer |

### 4.3 低優先

| 項目 | 來源 | 說明 |
|------|------|------|
| UI 文字 i18n | PM-NEW-03 | 確認框、按鈕文字在地化 |
| API Key 加密儲存 | v0.3.0 BUG-07 | 平台 keychain |
| `~` 路徑替換改用 strip_prefix | v0.3.0 BUG-08 | 跨平台安全 |
| 前端測試覆蓋 | — | 目前零前端測試 |
| 統一 Svelte 風格 | v0.3.0 R-07 | 全部 runes 或全部 legacy |

---

## 5. 測試覆蓋率觀察

### Rust 端

| 模組 | v0.3.0 | v0.4.0 | 說明 |
|------|--------|--------|------|
| `storage.rs` | 0 tests | **17 tests** | parse/serialize/extract/id 全覆蓋 |
| `commands.rs` | 0 tests | 0 tests | 需要 mock（AI API, config） |
| `claude.rs` | 0 tests | 0 tests | 需要 mock（HTTP client） |
| `config.rs` | 0 tests | 0 tests | 低複雜度 |

**進步顯著**：從零測試到 17 個核心測試，覆蓋了最容易出錯的 frontmatter 解析。Round-trip 測試是最有價值的類型 — 如果 serialize → parse 能還原，幾乎可以保證正確性。

### 前端

| 模組 | 測試 | 說明 |
|------|------|------|
| 全部 .svelte | 0 | 無單元/整合測試 |

**建議**：v0.5.0 考慮為 dirty check 邏輯、language list 編輯邏輯補上前端測試。

---

## 6. 程式碼品質觀察

### 6.1 正面

- **Title 引號保護**的實作乾淨：serialize 加引號 + escape，parse 單層 unquote + unescape，round-trip 測試驗證
- **Frontmatter delimiter** 修正精確：`\n---\n` + EOF fallback 處理得當
- **翻譯平行化**用 `join_all` 乾淨，個別失敗不影響整體
- **CorrectionResult struct** 讓 Rust ↔ TS 的型別契約清晰
- **Fire-and-forget** 主題存檔是正確的 optimistic update 模式

### 6.2 需注意

- **Svelte legacy subscribe 模式**遍佈整個 app。這在 Svelte 5 中雖然仍可用，但與 Settings.svelte 的 `$state`/`$effect` 風格不一致。未來統一方向建議往 runes 走。
- **isDirty 的 subscribe 觸發邏輯**是最脆弱的部分，容易因為新增 store 更新而引入 bug。
- **EntryList 從 `<button>` 改為 `<div role="button">`**是正確的 HTML 修正，但要注意鍵盤 navigation — 目前只監聽 Enter，未處理 Space（`<button>` 原生支援 Space 觸發）。

---

## 7. 總結

v0.4.0 是一個**紮實的品質提升版本**。12 項計畫全部交付，5 個 v0.3.0 bug 修正、4 個新功能、多語翻譯平行化、17 個 Rust 測試。

**最大亮點**：AI 修正附帶解釋（#5），真正把 app 從「修正工具」升級為「語言學習助手」。

**最大風險**：dirty check 的 subscribe 時序和 explanation 不清空是應該優先處理的問題。

**建議下一步**：
1. 先修 NEW-BUG-05（explanation 不清空）— 一行修正
2. 先修 NEW-BUG-03（刪除語言防護）— 幾行修正
3. 再規劃 v0.5.0（window close dirty check、i18n、前端測試）
