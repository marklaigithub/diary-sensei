# Review Report — diary-sensei v0.4.0 (Layered Review)

**日期**：2026-02-25
**方法**：分層式 Review（Layer 0 自動掃描 → Layer 1 Haiku 驗證 → Layer 2 Sonnet PM+QA）
**範圍**：v0.3.0 → v0.4.0（19 個檔案，+1590/-158 行）

## 總覽

| 層級 | 來源 | 發現數 |
|------|------|--------|
| Layer 0 | 自動掃描（Grep/Glob） | 2 個（1🟡 1🟢） |
| Layer 1 | 結構驗證（Haiku） | ✅ 全部通過 |
| Layer 2 PM | 產品視角（Sonnet） | 8 個（1🔴 4🟡 3🟢） |
| Layer 2 QA | 品質視角（Sonnet） | 12 個（2🔴 6🟡 3🟢） |
| **合併去重** | | **3🔴 8🟡 5🟢** |

## Layer 0 自動掃描結果

- 安全風險：✅ 全部乾淨（無 eval、innerHTML、硬寫密鑰）
- Debug 殘留：✅ 全部乾淨
- 技術債：✅ 無 TODO/FIXME
- 🟡 `App.svelte:149` — `as any` 型別強制轉換
- 🟢 `styles.css:114,119` — `!important`（列印樣式）
- Rust unwrap()：全在 #[cfg(test)] 中，合理

## Layer 1 結構驗證

- Build：✅ cargo test 17 項全通過
- Review 文件：✅ 存在 v0.4.0 PM+QA 報告
- 版本一致性：✅ package.json / tauri.conf.json 一致
- Round-trip 測試：✅ 4 個序列化-反序列化測試

## 🔴 Blocker（3 個）

### 1. skipDirtyTracking 例外路徑殘留 true
**`App.svelte:108-115`** — onMount 的 catch block 沒有重置 skipDirtyTracking，載入失敗後 dirty check 永久失效。
→ catch block 加 `skipDirtyTracking = false` + `error.set(...)`

### 2. ~ 路徑展開 fallback 產生錯誤路徑
**`storage.rs:40-41`** — `dirs::home_dir()` 失敗時回傳空字串，`~/Documents` 變 `/Documents`。
→ 改用 `.ok_or_else(|| "Cannot find home directory")?`

### 3. 語言刪除按鈕 tooltip 顯示「Cancel」
**`Settings.svelte:241`** — `title={$t('settings.cancel')}` 應為刪除相關 i18n key。
→ 新增 `settings.deleteLanguage` i18n key

## 🟡 Should Fix（8 個）

1. `App.svelte:149` — `as any` 遮蔽無效 mode 值，應加 runtime 驗證
2. `App.svelte:611,624` — AI 處理中刪除/儲存按鈕未 disabled
3. `App.svelte:279` — 翻譯語言全刪光後按翻譯無提示
4. `App.svelte:283` — 5 處 `e.toString()` 原始錯誤顯示給使用者
5. `App.svelte:381` — 刪除確認未提及「不可復原」
6. `Settings.svelte:65` — 新增語言未檢查 code 重複
7. `App.svelte:117` — loadEntries 失敗靜默
8. `App.svelte:450` — `subscribe()()` 應改用 `get()`

## 🟢 Nice to Have（5 個）

- `isLoading` store 是 dead code
- 翻譯失敗訊息跟正常翻譯視覺無區分
- commands.rs 每次呼叫重讀 config
- clipboard API 未處理 rejection
- Claude API 無 rate limit / retry

## 測試覆蓋

| Source File | 有 Test？ | 覆蓋狀況 |
|-------------|:---------:|---------|
| storage.rs | ✅ 18 個 | Happy + Error + Edge |
| commands.rs | ❌ | 最大缺口：AI 回應解析無測試 |
| claude.rs | ❌ | API 呼叫層 |
| 6 個 .svelte | ❌ | 無前端測試 |

## 回歸風險

| 等級 | 風險 |
|------|------|
| 🔴 | skipDirtyTracking 在 12+ 處設定，exception path 可能殘留 |
| 🔴 | isSectionHeader regex 可能誤判使用者日記 |
| 🟡 | 翻譯失敗字串混入正常翻譯結果 |
| 🟡 | 刪除語言後既有日記語言設定成孤兒 |
