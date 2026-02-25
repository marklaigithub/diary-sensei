# Committee QA Test Scenarios - diary-sensei
Date: 2026-02-25

## Tier 1: Universal Consensus (10/10 sources agree)

### TS-01: Translation Mode Editor Clearing
- **Trigger**: Switch from Writing to Translation mode
- **Expected**: User understands their content is preserved
- **Actual**: Editor clears with no visual indication, users panic thinking content is deleted
- **Test**: Switch modes, verify visual feedback exists (toast/banner saying "diary content preserved, this is a scratch pad")
- **Found by**: ALL sources (Claude ×6, Ollama ×4)
- **Category**: UX / Critical

### TS-02: 5-Second Undo Countdown
- **Trigger**: Apply AI correction, undo button appears for 5 seconds
- **Expected**: User has adequate time and awareness to undo
- **Actual**: Button disappears silently, no way to recover after 5 seconds
- **Test**: Apply correction → wait 6 seconds → verify no undo path exists; also test: user is looking at sidebar when undo appears → misses it entirely
- **Found by**: 8/10 sources
- **Category**: UX / Critical

## Tier 2: Strong Consensus (5+ sources)

### TS-03: Stale AI Result Pollution
- **Trigger**: Submit AI request on Entry A → switch to Entry B before result returns
- **Expected**: Entry B shows clean state, Entry A's result is discarded
- **Actual**: Entry A's AI result displays alongside Entry B's content
- **Test**: Start correction on entry A, immediately click entry B, wait for AI to return → verify translations store is NOT updated with stale result
- **Found by**: Claude QA, Haiku clumsy, Sonnet heavy-user, llama, mistral
- **Category**: Race Condition / Critical
- **Technical**: Need request token or AbortController in handleSubmit (App.svelte:268-300)

### TS-04: No Onboarding Flow
- **Trigger**: First app launch
- **Expected**: Guided setup (choose language, configure AI backend)
- **Actual**: Empty 3-column interface with no guidance
- **Test**: Fresh install → verify first-run experience guides user through: language selection, AI backend setup, basic mode explanation
- **Found by**: Claude Haiku first-time, phi4-mini, mistral, Opus
- **Category**: UX / High

### TS-05: API Key Plaintext Storage
- **Trigger**: Save Claude API key in settings
- **Expected**: Key stored securely (OS keychain)
- **Actual**: Plaintext in ~/Documents/diary-language/config.json
- **Test**: Save API key → read config.json → verify key is NOT in plaintext
- **Found by**: Claude Opus, QA, mistral
- **Category**: Security / High

### TS-06: Non-Atomic File Write
- **Trigger**: Save entry (any mode)
- **Expected**: Either full save or no save (atomic)
- **Actual**: fs::write can produce partial writes on crash/power loss
- **Test**: Simulate interrupted write → verify file is either complete or unchanged
- **Found by**: Claude QA, Opus, Sonnet heavy-user
- **Category**: Data Integrity / High
- **Technical**: storage.rs:194 - change to write-to-temp + atomic rename

### TS-07: Search Performance Degradation
- **Trigger**: Search with 300+ entries
- **Expected**: Results within 200ms
- **Actual**: Full directory scan with no index, scales linearly
- **Test**: Generate 500 test entries → measure search latency → verify under 500ms
- **Found by**: Claude Sonnet heavy-user, QA, Opus
- **Category**: Performance / High
- **Technical**: storage.rs:93-154 - needs indexing

### TS-08: Language Not Bound to Entry
- **Trigger**: Open old Japanese entry while global language is set to Korean
- **Expected**: Auto-switch correction language to match entry
- **Actual**: Uses global selectedLangsVal[0], may correct Japanese with Korean rules
- **Test**: Create JA entry → switch to KO → reopen JA entry → submit correction → verify language parameter is "ja"
- **Found by**: Claude Sonnet multi-lang, heavy-user, phi4-mini, mistral
- **Category**: UX / High

## Tier 3: Unique Findings (1-2 sources, need validation)

### TS-09: AI Backend Quality Inconsistency
- **Trigger**: Switch between Ollama and Claude API backends
- **Expected**: User understands quality may differ
- **Actual**: No indication of which backend is active; quality varies dramatically
- **Test**: Run same text through Ollama (gemma2:9b) and Claude (haiku) → compare result quality → verify UI shows active backend prominently
- **Found by**: phi4-mini (UNIQUE)
- **Category**: UX / Medium
- **Validation needed**: Is this a real user pain point or theoretical?

### TS-10: UI Responsiveness / Input Lag
- **Trigger**: Type in editor with large entry loaded
- **Expected**: Keystroke response < 50ms
- **Actual**: Unknown - no performance testing exists
- **Test**: Load 5000-char entry → measure input latency → verify under 100ms
- **Found by**: llama3.2:3b (UNIQUE)
- **Category**: Performance / Medium

### TS-11: AI Over-Correction Hindering Learning
- **Trigger**: AI aggressively corrects grammar
- **Expected**: User learns from corrections
- **Actual**: User may become dependent on AI, not actually learning
- **Test**: N/A (design question, not testable bug)
- **Found by**: mistral:7b (UNIQUE)
- **Category**: Product Design / Discussion needed
- **Design question**: Should there be a "correction intensity" setting?

### TS-12: Mark-for-Review Feature Missing
- **Trigger**: User wants to flag specific corrections for later study
- **Expected**: Can bookmark/tag corrections
- **Actual**: No review mechanism; corrections are one-time events
- **Found by**: llama3.2:3b (UNIQUE)
- **Category**: Feature Gap / Medium

### TS-13: Ollama Model Staleness
- **Trigger**: Using app for months without updating Ollama model
- **Expected**: Correction quality remains consistent
- **Actual**: Local model doesn't auto-update; quality static while user improves
- **Found by**: gemma2:9b (UNIQUE)
- **Category**: Product Design / Low priority

### TS-14: Translation Error Messages Saved to File
- **Trigger**: Translation fails (network error, timeout) → user saves entry
- **Expected**: Error messages NOT persisted
- **Actual**: "[Translation failed: Connection refused]" saved as translation content
- **Test**: Disconnect network → translate → save → reopen → verify no error strings in file
- **Found by**: Claude Sonnet QA (UNIQUE)
- **Category**: Data Integrity / Medium
- **Technical**: commands.rs:172-176

### TS-15: Double handleModeSwitch Trigger
- **Trigger**: Select entry while in Translation mode, target entry also has translation mode
- **Expected**: Single clean mode transition
- **Actual**: appMode.set fires twice → handleModeSwitch runs twice → savedWritingContent corrupted
- **Test**: In translation mode → click entry that was saved in translation mode → verify savedWritingContent is correct
- **Found by**: Claude Sonnet QA (UNIQUE)
- **Category**: State Bug / High
- **Technical**: App.svelte handleEntrySelect lines 139-169

### TS-16: CSP Over-Permission in Ollama Mode
- **Trigger**: Using Ollama mode
- **Expected**: No allowed connections to external APIs
- **Actual**: CSP still allows connect-src to api.anthropic.com
- **Test**: In Ollama mode → verify no CSP allowance for external API domains
- **Found by**: Claude Opus (UNIQUE)
- **Category**: Security / Medium
- **Technical**: tauri.conf.json CSP line

### TS-17: No Privacy Warning on Provider Switch
- **Trigger**: Switch AI provider from Ollama to Claude in settings
- **Expected**: Warning dialog about data leaving machine
- **Actual**: Silent switch, no confirmation
- **Test**: Switch from Ollama to Claude → verify confirmation dialog appears with clear privacy implications
- **Found by**: Claude Opus, mistral
- **Category**: Privacy / High

### TS-18: save_image Path Traversal
- **Trigger**: save_image called with crafted filename like "../../evil.sh"
- **Expected**: Filename sanitized, restricted to entry's image directory
- **Actual**: Path joins filename directly without validation
- **Test**: Call save_image with "../test.txt" → verify file NOT created outside image directory
- **Found by**: Claude Sonnet QA (UNIQUE)
- **Category**: Security / High
- **Technical**: storage.rs save_image_to_disk

### TS-19: Config Corruption Silent Fallback
- **Trigger**: config.json becomes corrupted
- **Expected**: Error shown to user, option to reset or locate backup
- **Actual**: Silently loads default config, entries_dir changes, "all entries disappear"
- **Test**: Corrupt config.json → launch app → verify error notification shown
- **Found by**: Claude Sonnet QA, Opus
- **Category**: Error Handling / High
- **Technical**: config.rs:97-105

### TS-20: Correction History Not Persistent
- **Trigger**: Reopen a previously corrected entry
- **Expected**: See original vs corrected comparison
- **Actual**: translations.set({}) clears correction results on load
- **Test**: Correct entry → save → close → reopen → verify correction results visible
- **Found by**: Claude Sonnet heavy-user (UNIQUE for this specific mechanism)
- **Category**: Feature Gap / Medium

## Divergence Scenarios (Committee Disagreement)

### DS-01: Fix the Bug vs Change the Design
- **Issue**: Translation mode save uses stale savedWritingContent
- **Camp A** (Claude): Fix the bug - keep design, ensure savedWritingContent stays current
- **Camp B** (llama3.2:3b): Change the design - let users save directly from Translation mode
- **Resolution needed**: Human decision on product direction

### DS-02: Technical Privacy vs Legal Privacy
- **Issue**: Privacy protection approach
- **Camp A** (Opus): Fix CSP, remove shell:default, dynamic permissions
- **Camp B** (mistral): GDPR compliance, third-party audit, encryption at rest
- **Resolution**: Both are valid layers; prioritize Camp A (immediate, free) then Camp B (longer-term)

### DS-03: Optimize AI vs Question AI
- **Issue**: AI correction value
- **Camp A** (Claude + phi4): Make corrections more accurate and consistent
- **Camp B** (mistral): Consider if aggressive correction helps or hinders learning
- **Resolution needed**: Human decision - add "correction intensity" setting?
