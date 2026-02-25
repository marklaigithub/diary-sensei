# Review Report — diary-sensei v0.5.0

**日期**：2026-02-25
**範圍**：v0.4.0 → v0.5.0 + uncommitted changes（25 files, +2127/-257）
**Review 方法**：分層式 Review（L0 自動掃描 → L1 結構驗證 → L2 PM+QA → L3 Opus 深度分析）

## 總覽

| 層級 | 來源 | 發現數 |
|------|------|--------|
| Layer 0 | 自動掃描（Grep/Glob） | 2（1🟡 1🟢） |
| Layer 1 | Build + Test | ✅ pass（1 build warning） |
| Layer 2 PM | 產品視角（Sonnet） | 1🔴 5🟡 2🟢 + 3 連動風險 |
| Layer 2 QA | 品質視角（Sonnet） | 4🟡 3🟢 + 2🔍 Opus 複審 |
| Layer 3 | 深度分析（Opus） | 1🟡 1🟢 |

**最終統計**：1🔴 8🟡 6🟢

---

## Layer 0 自動掃描結果

### 安全風險
- ✅ 無安全風險（無 eval、innerHTML、硬寫密鑰等）

### Debug 殘留
- ✅ 無 console.log、debugger

### Error Handling
- 🟡 `src/App.svelte:150` — `as any` 強制轉型（appMode.set(entry.meta.mode as any)）

### 技術債
- ✅ 無 TODO/FIXME/HACK

### 程式碼品質
- 🟢 `src/styles.css:114,119` — `!important`（print media queries，可接受）

### 測試覆蓋

| Source | 有 Test？ |
|--------|:--------:|
| src-tauri/src/storage.rs | ✅ 17 tests |
| src/App.svelte | ❌ |
| src/lib/*.svelte (8 files) | ❌ |
| src/i18n.ts | ❌ |

### Build 警告
- 🟡 ThemeSwitcher.svelte:14 — `configVal` 未用 `$state()` 宣告（誤報——使用 Svelte 4 store subscription 模式，非反應性需求）

---

## Layer 1 結構驗證

| 項目 | 結果 |
|------|------|
| 前端 build | ✅ 通過（878ms） |
| Rust tests | ✅ 17/17 通過 |
| Review 文件 | ✅ v0.4.0 review 存在 |

---

## 🔴 Blocker（1 個，必須修）

| # | 來源 | 位置 | 問題 |
|---|------|------|------|
| 1 | PM | `App.svelte:294` | 日期驗證錯誤訊息硬寫英文 `'Invalid date...'`，在非英文介面下破壞 i18n 體驗 |

---

## 🟡 Should Fix（8 個）

| # | 來源 | 位置 | 問題 |
|---|------|------|------|
| 1 | PM | `App.svelte:156,327,400,533` + `Settings:111` | 多處錯誤訊息硬寫英文（load/save/delete/print failed） |
| 2 | PM+QA | `App.svelte:266` | `explanationLanguage` 直接讀 localStorage，跳過 svelte-i18n 的 `locale` store，雙 source of truth |
| 3 | PM | `App.svelte:302` | 預設標題 `"diary"` 硬寫英文，應 i18n 化 |
| 4 | PM | `App.svelte:61` | `prevMode` 初始值硬寫 `'correction'`，與 store 預設值耦合 |
| 5 | QA | `App.svelte:150` | `as any` 無執行期驗證，非法 mode 值會靜默失效 |
| 6 | QA | `App.svelte:136-137` | `handleEntrySelect` 雙重觸發 `handleModeSwitch`，翻譯模式下選取 translation 日記時流程不乾淨 |
| 7 | QA | `Translation:25` + `Editor:77` | `clipboard.writeText` 無 error handling |
| 8 | L3 Opus | `App.svelte:215-238` | handleModeSwitch 內部管理 skipDirtyTracking，與呼叫端的保護區衝突——應將管理權移到呼叫端 |

---

## 🟢 Nice to Have（6 個）

| # | 來源 | 問題 |
|---|------|------|
| 1 | PM | Tauri bridge camelCase/snake_case 命名風格一致性 |
| 2 | PM | Translation.svelte 複數 key 對中日文無意義 |
| 3 | QA | `'diary-sensei-locale'` localStorage key 應抽常數 |
| 4 | QA | 搜尋失敗時用戶無回饋（僅 console.error） |
| 5 | QA | 翻譯失敗的 `[Translation failed: ...]` 格式不夠顯眼 |
| 6 | L3 Opus | skipDirtyTracking 保護區加防禦性註解 |

---

## 功能連動風險

| # | 情境 | 風險 |
|---|------|------|
| 1 | 翻譯模式切換月份 | scratch pad 內容不清空，切回寫作模式可能看到舊內容 |
| 2 | closedEntryIds 月份切換 | 換頁再換回，手動關閉的文章重新出現 |
| 3 | 搜尋 vs closedEntryIds | 已關閉的文章在搜尋結果仍顯示 |

---

## Layer 3 深度分析（Opus）

### handleModeSwitch 多觸發路徑

**判定：功能正確但脆弱（🟡）**

handleEntrySelect 在翻譯模式下會觸發兩次 handleModeSwitch：
1. L137 `appMode.set('correction')` → TRIGGER 1
2. L150 `appMode.set(entry.meta.mode)` → TRIGGER 2

TRIGGER 1 的效果被載入階段覆蓋，TRIGGER 2 產生正確的最終狀態。但 TRIGGER 2 的 handleModeSwitch 在 L228 把 `skipDirtyTracking` 設回 `false`，提前解除了 L143 開啟的保護區。目前不觸發 dirty（只追蹤 editorContent 和 entryTitle），但未來擴展 dirty tracking 時會踩坑。

**建議**：將 skipDirtyTracking 管理權從 handleModeSwitch 移到呼叫端。

### skipDirtyTracking race condition

**判定：目前安全（🟢）**

所有 skipDirtyTracking 保護區都是同步的（true → sync stores → false），JavaScript 單執行緒保護了不可能在保護區內被中斷。快速連點場景追蹤確認最終狀態正確。

唯一例外是 `onMount` 在保護區內有 await，但只在啟動時執行一次。

**建議**：加防禦性註解。

---

## 與 v0.4.0 Review 比較

| 指標 | v0.4.0 | v0.5.0 |
|------|--------|--------|
| 🔴 Blocker | 3 | 1 |
| 🟡 Should Fix | 8 | 8 |
| 🟢 Nice to Have | 5 | 6 |
| Blocker 性質 | 核心邏輯（skipDirtyTracking catch、storage path） | i18n 遺漏（體驗不一致） |
| 安全掃描 | `as any` 2 處 | `as any` 1 處 |
| 測試覆蓋 | Rust 17 tests | Rust 17 tests（前端仍無） |

品質趨勢：Blocker 從「會壞」降為「體驗不一致」，整體改善。

---

## 建議下一步

1. 修 🔴 Blocker：日期驗證訊息 i18n
2. 修 🟡 #1-5：所有硬寫英文訊息 + source of truth + prevMode
3. 修 🟡 #8：handleModeSwitch skipDirtyTracking 重構
4. 修 🟡 #6-7：雙重觸發 + clipboard error handling
5. 加防禦性註解（🟢 #6）
