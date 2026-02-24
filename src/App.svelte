<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import Calendar from './lib/Calendar.svelte';
  import EntryList from './lib/EntryList.svelte';
  import Editor from './lib/Editor.svelte';
  import Correction from './lib/Correction.svelte';
  import Translation from './lib/Translation.svelte';
  import Settings from './lib/Settings.svelte';
  import ModeSelector from './lib/ModeSelector.svelte';
  import ThemeSwitcher from './lib/ThemeSwitcher.svelte';
  import {
    config, showSettings, appMode, currentEntry,
    selectedYear, selectedMonth, entries, currentDate,
    editorContent, entryTitle, translations, isProcessing,
    error, isLoading, currentEntryId, selectedTargetLanguages
  } from './lib/store';
  import type { AppConfig, EntryListItem, DiaryEntry } from './lib/types';

  let configVal: AppConfig;
  let showSettingsVal: boolean;
  let modeVal: string;
  let currentEntryVal: any;
  let yearVal: number;
  let monthVal: number;
  let editorVal: string;
  let titleVal: string;
  let translationsVal: Record<string, string>;
  let processingVal: boolean;
  let errorVal: string;
  let selectedLangsVal: string[];

  config.subscribe(v => configVal = v);
  showSettings.subscribe(v => showSettingsVal = v);
  appMode.subscribe(v => modeVal = v);
  currentEntry.subscribe(v => currentEntryVal = v);
  selectedYear.subscribe(v => yearVal = v);
  selectedMonth.subscribe(v => monthVal = v);
  editorContent.subscribe(v => editorVal = v);
  entryTitle.subscribe(v => titleVal = v);
  translations.subscribe(v => translationsVal = v);
  isProcessing.subscribe(v => processingVal = v);
  error.subscribe(v => errorVal = v);
  selectedTargetLanguages.subscribe(v => selectedLangsVal = v);

  onMount(async () => {
    try {
      const cfg: AppConfig = await invoke('load_config');
      config.set(cfg);
      document.documentElement.dataset.theme = cfg.theme;
      await loadEntries();
    } catch (e) {
      console.error('Failed to load config:', e);
    }
  });

  async function loadEntries() {
    try {
      const items: EntryListItem[] = await invoke('list_entries', {
        year: yearVal,
        month: monthVal,
      });
      entries.set(items);
    } catch (e) {
      console.error('Failed to load entries:', e);
    }
  }

  async function handleEntrySelect(event: CustomEvent<string>) {
    const id = event.detail;
    currentEntryId.set(id);
    try {
      const entry: DiaryEntry = await invoke('read_entry', { id });
      currentEntry.set(entry);
      currentDate.set(entry.meta.date);
      editorContent.set(entry.original);
      entryTitle.set(entry.meta.title);
      translations.set(entry.translations);
      appMode.set(entry.meta.mode as any);
      selectedTargetLanguages.set(entry.meta.languages);
    } catch (e) {
      error.set('Failed to load entry');
    }
  }

  async function handleDateSelect(event: CustomEvent<string>) {
    const date = event.detail;
    currentDate.set(date);
    currentEntryId.set(null);
    currentEntry.set(null);
    editorContent.set('');
    entryTitle.set('');
    translations.set({});
  }

  async function handleMonthChange(event: CustomEvent<{year: number, month: number}>) {
    selectedYear.set(event.detail.year);
    selectedMonth.set(event.detail.month);
    await loadEntries();
  }

  async function handleSubmit() {
    if (!editorVal.trim()) return;
    isProcessing.set(true);
    error.set('');

    try {
      if (modeVal === 'correction') {
        const result: string = await invoke('correct_text', {
          text: editorVal,
          language: selectedLangsVal[0] || configVal.default_language,
        });
        translations.set({ [selectedLangsVal[0] || configVal.default_language]: result });
      } else {
        const results: Record<string, string> = await invoke('translate_text', {
          text: editorVal,
          targetLanguages: selectedLangsVal,
        });
        translations.set(results);
      }
    } catch (e: any) {
      error.set(e.toString());
    } finally {
      isProcessing.set(false);
    }
  }

  async function handleSave() {
    let dateVal: string = '';
    currentDate.subscribe(v => dateVal = v)();
    let entryIdVal: string | null = null;
    currentEntryId.subscribe(v => entryIdVal = v)();

    try {
      const savedId: string = await invoke('save_entry', {
        id: entryIdVal,
        title: titleVal || `${dateVal} diary`,
        date: dateVal,
        mode: modeVal,
        languages: selectedLangsVal,
        original: editorVal,
        translations: translationsVal,
      });
      currentEntryId.set(savedId);
      await loadEntries();
      error.set('');
    } catch (e: any) {
      error.set('Save failed: ' + e.toString());
    }
  }

  function handleNewEntry() {
    currentEntryId.set(null);
    currentEntry.set(null);
    editorContent.set('');
    entryTitle.set('');
    translations.set({});
  }

  function handleApplyCorrection() {
    const langKey = selectedLangsVal[0] || configVal.default_language;
    const corrected = translationsVal[langKey];
    if (corrected) {
      editorContent.set(corrected);
    }
  }

  async function handlePrint() {
    try {
      await invoke('print_page');
    } catch (e: any) {
      error.set('Print failed: ' + e.toString());
    }
  }
</script>

<div class="app-layout">
  <!-- Sidebar -->
  <aside class="sidebar">
    <div class="sidebar-header">
      <h1 class="app-title">Diary Language</h1>
      <div class="sidebar-actions">
        <ThemeSwitcher />
        <button class="icon-btn" onclick={() => showSettings.set(true)} title="Settings">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
          </svg>
        </button>
      </div>
    </div>

    <Calendar
      on:dateSelect={handleDateSelect}
      on:monthChange={handleMonthChange}
    />

    <EntryList on:select={handleEntrySelect} />
  </aside>

  <!-- Main content -->
  <main class="main-content">
    {#if showSettingsVal}
      <Settings />
    {:else}
      <div class="editor-panel">
        <div class="editor-header">
          <div class="editor-header-top">
            <ModeSelector />
            <button class="btn btn-new-entry" onclick={handleNewEntry} title="New entry for current date">
              + New
            </button>
          </div>
          <input
            type="text"
            class="title-input"
            placeholder="Title..."
            value={titleVal}
            oninput={(e) => entryTitle.set((e.target as HTMLInputElement).value)}
          />
        </div>

        <Editor />

        <div class="editor-footer">
          {#if errorVal}
            <div class="error-msg">{errorVal}</div>
          {/if}
          <div class="action-buttons">
            <button class="btn btn-primary" onclick={handleSubmit} disabled={processingVal}>
              {#if processingVal}
                Processing...
              {:else if modeVal === 'correction'}
                Submit Correction
              {:else}
                Submit Translation
              {/if}
            </button>
            <button class="btn btn-secondary" onclick={handleSave}>
              Save
            </button>
          </div>
        </div>
      </div>
    {/if}
  </main>

  <!-- Result panel -->
  <aside class="result-panel">
    {#if modeVal === 'correction'}
      <Correction on:apply={handleApplyCorrection} />
    {:else}
      <Translation />
    {/if}

    {#if Object.keys(translationsVal).length > 0}
      <div class="panel-footer">
        <button class="btn btn-outline no-print" onclick={handlePrint}>
          Export PDF
        </button>
      </div>
    {/if}
  </aside>
</div>

<style>
  .app-layout {
    display: grid;
    grid-template-columns: 260px 1fr 1fr;
    height: 100vh;
    overflow: hidden;
  }

  .sidebar {
    background: var(--bg-sidebar);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px;
    border-bottom: 1px solid var(--border);
  }

  .app-title {
    font-size: 16px;
    font-weight: 600;
  }

  .sidebar-actions {
    display: flex;
    gap: 4px;
    align-items: center;
  }

  .icon-btn {
    padding: 6px;
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    transition: all 0.2s;
  }

  .icon-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .main-content {
    display: flex;
    flex-direction: column;
    overflow: hidden;
    border-right: 1px solid var(--border);
  }

  .editor-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .editor-header {
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-light);
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .editor-header-top {
    display: flex;
    align-items: flex-start;
    gap: 8px;
  }

  .editor-header-top :global(.mode-selector) {
    flex: 1;
  }

  .btn-new-entry {
    padding: 6px 12px;
    border-radius: var(--radius);
    font-size: 13px;
    font-weight: 500;
    border: 1px solid var(--border);
    color: var(--text-secondary);
    white-space: nowrap;
    transition: all 0.15s;
    align-self: flex-start;
  }

  .btn-new-entry:hover {
    border-color: var(--accent);
    color: var(--text-primary);
    background: var(--bg-hover);
  }

  .title-input {
    width: 100%;
    border: none;
    background: transparent;
    font-size: 18px;
    font-weight: 500;
    padding: 4px 0;
    outline: none;
  }

  .title-input::placeholder {
    color: var(--text-muted);
  }

  .editor-footer {
    padding: 12px 16px;
    border-top: 1px solid var(--border-light);
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .error-msg {
    color: var(--diff-removed-text);
    background: var(--diff-removed-bg);
    padding: 8px 12px;
    border-radius: var(--radius-sm);
    font-size: 13px;
  }

  .action-buttons {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  .btn {
    padding: 8px 16px;
    border-radius: var(--radius);
    font-weight: 500;
    transition: all 0.2s;
  }

  .btn-primary {
    background: var(--btn-primary);
    color: var(--btn-primary-text);
  }

  .btn-primary:hover:not(:disabled) {
    background: var(--btn-primary-hover);
  }

  .btn-primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-secondary {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .btn-secondary:hover {
    background: var(--bg-active);
  }

  .btn-outline {
    border: 1px solid var(--border);
    color: var(--text-secondary);
  }

  .btn-outline:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .result-panel {
    background: var(--bg-panel);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
  }

  .panel-footer {
    padding: 12px 16px;
    border-top: 1px solid var(--border-light);
    display: flex;
    justify-content: flex-end;
  }
</style>
