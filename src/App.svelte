<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { confirm } from '@tauri-apps/plugin-dialog';
  import { onMount } from 'svelte';
  import { t, locale } from 'svelte-i18n';
  import { get } from 'svelte/store';
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
    error, isLoading, currentEntryId, selectedTargetLanguages,
    isDirty, explanation, searchResults, closedEntryIds, correctionOriginal
  } from './lib/store';
  import type { AppConfig, EntryListItem, DiaryEntry, CorrectionResult } from './lib/types';

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
  let dateVal: string = '';
  let dirtyVal: boolean = false;
  let editorRef: Editor;
  let currentEntryIdVal: string | null = null;

  // Undo state for Accept Correction
  let undoState: {
    previousContent: string;
    previousTranslations: Record<string, string>;
    previousExplanation: string | null;
    countdown: number;
  } | null = null;
  let undoTimer: ReturnType<typeof setInterval> | null = null;

  // Print mode for Export PDF dialog
  let printMode: 'original' | 'full' | null = null;
  let showPrintDialog = false;

  // Saved writing mode state (preserved when in translation mode)
  let savedWritingContent: string = '';
  let savedWritingTitle: string = '';
  let savedWritingDirty: boolean = false;

  config.subscribe(v => configVal = v);
  showSettings.subscribe(v => showSettingsVal = v);
  let prevMode: string = get(appMode);
  appMode.subscribe(v => {
    if (v !== prevMode) {
      // Guard: if skipDirtyTracking is already true (caller manages it), don't interfere
      const wasSkipping = skipDirtyTracking;
      if (!wasSkipping) skipDirtyTracking = true;
      handleModeSwitch(prevMode, v);
      if (!wasSkipping) skipDirtyTracking = false;
      prevMode = v;
    }
    modeVal = v;
  });
  currentEntry.subscribe(v => currentEntryVal = v);
  selectedYear.subscribe(v => yearVal = v);
  selectedMonth.subscribe(v => monthVal = v);
  editorContent.subscribe(v => editorVal = v);
  entryTitle.subscribe(v => titleVal = v);
  translations.subscribe(v => translationsVal = v);
  isProcessing.subscribe(v => processingVal = v);
  error.subscribe(v => errorVal = v);
  selectedTargetLanguages.subscribe(v => selectedLangsVal = v);
  currentDate.subscribe(v => dateVal = v);
  isDirty.subscribe(v => dirtyVal = v);
  currentEntryId.subscribe(v => currentEntryIdVal = v);

  // Track editor/title changes to set dirty flag
  // IMPORTANT: skipDirtyTracking guard zones MUST be synchronous (no await inside)
  let skipDirtyTracking = false;
  editorContent.subscribe(() => { if (!skipDirtyTracking) isDirty.set(true); });
  entryTitle.subscribe(() => { if (!skipDirtyTracking) isDirty.set(true); });

  function clearUndoState() {
    if (undoTimer) {
      clearInterval(undoTimer);
      undoTimer = null;
    }
    undoState = null;
  }

  async function checkDirty(): Promise<boolean> {
    if (!dirtyVal) return true;
    return await confirm(get(t)('dialog.unsavedMessage'), {
      title: get(t)('dialog.unsavedTitle'),
      kind: 'warning',
    });
  }

  onMount(async () => {
    try {
      const cfg: AppConfig = await invoke('load_config');
      config.set(cfg);
      document.documentElement.dataset.theme = cfg.theme;
      skipDirtyTracking = true;
      await loadEntries();
      // Start with all entries closed — user opens via calendar interaction
      const allIds = new Set(get(entries).map((e: EntryListItem) => e.id));
      closedEntryIds.set(allIds);
      isDirty.set(false);
      skipDirtyTracking = false;
    } catch (e) {
      console.error('Failed to load config:', e);
      skipDirtyTracking = false;
    }
  });

  async function loadEntries() {
    try {
      const items: EntryListItem[] = await invoke('list_entries', {
        year: yearVal,
        month: monthVal,
      });
      entries.set(items);
      closedEntryIds.set(new Set()); // Reset when loading new month
    } catch (e) {
      console.error('Failed to load entries:', e);
    }
  }

  async function handleEntrySelect(event: CustomEvent<string>) {
    if (!await checkDirty()) return;
    clearUndoState();
    const id = event.detail;
    // If in translation mode, switch back to writing mode before loading
    if (modeVal === 'translation') {
      appMode.set('correction');
    }
    currentEntryId.set(id);
    closedEntryIds.update(s => { const next = new Set(s); next.delete(id); return next; });
    try {
      const entry: DiaryEntry = await invoke('read_entry', { id });
      skipDirtyTracking = true;
      explanation.set(null);
      correctionOriginal.set('');
      currentEntry.set(entry);
      currentDate.set(entry.meta.date);
      editorContent.set(entry.original);
      entryTitle.set(entry.meta.title);
      const validMode = (entry.meta.mode === 'translation') ? 'translation' : 'correction';
      // In correction mode, don't load stale correction results — they're ephemeral
      translations.set(validMode === 'translation' ? entry.translations : {});
      appMode.set(validMode);
      selectedTargetLanguages.set(entry.meta.languages);
      isDirty.set(false);
      skipDirtyTracking = false;
    } catch (e) {
      skipDirtyTracking = false;
      error.set(get(t)('error.loadFailed'));
    }
  }

  async function handleDateSelect(event: CustomEvent<string>) {
    if (!await checkDirty()) return;
    clearUndoState();
    // If in translation mode, switch back to writing mode
    if (modeVal === 'translation') {
      appMode.set('correction');
    }
    const date = event.detail;
    skipDirtyTracking = true;
    explanation.set(null);
    correctionOriginal.set('');
    currentDate.set(date);
    currentEntryId.set(null);
    currentEntry.set(null);
    editorContent.set('');
    entryTitle.set('');
    translations.set({});
    isDirty.set(false);
    skipDirtyTracking = false;
    // Show only entries for the selected date
    const currentEntries = get(entries);
    const allIds = new Set(currentEntries.map(e => e.id));
    const dateEntryIds = currentEntries.filter(e => e.date === date).map(e => e.id);
    dateEntryIds.forEach(id => allIds.delete(id));
    closedEntryIds.set(allIds);
  }

  async function handleDateInputChange(newDate: string) {
    if (!await checkDirty()) return;
    clearUndoState();
    skipDirtyTracking = true;
    explanation.set(null);
    correctionOriginal.set('');
    currentDate.set(newDate);
    currentEntryId.set(null);
    currentEntry.set(null);
    editorContent.set('');
    entryTitle.set('');
    translations.set({});
    isDirty.set(false);
    skipDirtyTracking = false;
    const parts = newDate.split('-');
    if (parts.length === 3) {
      const newYear = parseInt(parts[0]);
      const newMonth = parseInt(parts[1]);
      selectedYear.set(newYear);
      selectedMonth.set(newMonth);
      await loadEntries();
    }
  }

  async function handleMonthChange(event: CustomEvent<{year: number, month: number}>) {
    selectedYear.set(event.detail.year);
    selectedMonth.set(event.detail.month);
    await loadEntries();
  }

  // NOTE: skipDirtyTracking is managed by the caller (appMode.subscribe guard),
  // NOT inside this function. This prevents premature guard zone termination.
  function handleModeSwitch(from: string, to: string) {
    if (from === 'correction' && to === 'translation') {
      // Switching to translation: save writing state, clear editor for scratch pad
      savedWritingContent = editorVal;
      savedWritingTitle = titleVal;
      savedWritingDirty = dirtyVal;

      clearUndoState();
      editorContent.set('');
      translations.set({});
      explanation.set(null);
      isDirty.set(false);
    } else if (from === 'translation' && to === 'correction') {
      // Switching back to writing: restore writing state
      editorContent.set(savedWritingContent);
      entryTitle.set(savedWritingTitle);
      translations.set({});
      explanation.set(null);
      isDirty.set(savedWritingDirty);
    }
  }

  function extractTargetSection(content: string): string {
    const lines = content.split('\n');
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i].trim();
      if (line.length === 0) continue;
      if (isSectionHeader(line)) {
        let cutPoint = i;
        while (cutPoint > 0 && lines[cutPoint - 1].trim() === '') {
          cutPoint--;
        }
        return lines.slice(0, cutPoint).join('\n').trim();
      }
    }
    return content;
  }

  async function handleSubmit() {
    if (!editorVal.trim()) return;
    isProcessing.set(true);
    error.set('');

    try {
      if (modeVal === 'correction') {
        const targetSection = extractTargetSection(editorVal);
        correctionOriginal.set(targetSection);
        const currentLocale = get(locale) || 'en';
        const uiLangMap: Record<string, string> = {
          'en': 'English', 'zh-TW': '繁體中文', 'ja': '日本語', 'ko': '한국어', 'it': 'Italiano',
        };
        const result: CorrectionResult = await invoke('correct_text', {
          text: targetSection,
          language: selectedLangsVal[0] || configVal.default_language,
          explanationLanguage: uiLangMap[currentLocale] || 'English',
        });
        translations.set({ [selectedLangsVal[0] || configVal.default_language]: result.corrected });
        explanation.set(result.explanation || null);
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
    // Validate date format before saving
    if (!/^\d{4}-\d{2}-\d{2}$/.test(dateVal)) {
      error.set(get(t)('error.invalidDate'));
      return;
    }

    try {
      // In translation mode, save the main diary content, not the scratch pad
      const contentToSave = modeVal === 'translation' ? savedWritingContent : editorVal;
      const titleToSave = modeVal === 'translation' ? savedWritingTitle : titleVal;
      const actualTitle = titleToSave || `${dateVal} ${get(t)('app.defaultDiaryTitle')}`;
      const savedId: string = await invoke('save_entry', {
        id: currentEntryIdVal,
        title: actualTitle,
        date: dateVal,
        mode: 'correction',  // Always save as writing mode
        languages: selectedLangsVal,
        original: contentToSave,
        translations: translationsVal,
        createdAt: currentEntryVal?.meta?.created_at || null,
      });
      currentEntryId.set(savedId);
      // Sync default title back to UI if it was empty
      if (!titleToSave) {
        skipDirtyTracking = true;
        entryTitle.set(actualTitle);
        if (modeVal === 'translation') {
          savedWritingTitle = actualTitle;
        }
        skipDirtyTracking = false;
      }
      await loadEntries();
      isDirty.set(false);
      error.set('');
    } catch (e: any) {
      error.set(get(t)('error.saveFailed', { values: { detail: e.toString() } }));
    }
  }

  function formatTimestamp(ts: string): string {
    // "2026-02-24T14:30:52" -> "2026/02/24 14:30"
    const [datePart, timePart] = ts.split('T');
    if (!timePart) return datePart;
    return `${datePart.replace(/-/g, '/')} ${timePart.slice(0, 5)}`;
  }

  async function handleNewEntry() {
    if (!await checkDirty()) return;
    clearUndoState();
    // If in translation mode, switch back to writing mode
    if (modeVal === 'translation') {
      appMode.set('correction');
    }
    skipDirtyTracking = true;
    explanation.set(null);
    correctionOriginal.set('');
    currentEntryId.set(null);
    currentEntry.set(null);
    editorContent.set('');
    entryTitle.set('');
    translations.set({});
    isDirty.set(false);
    skipDirtyTracking = false;
  }

  async function handleEntryClose(event: CustomEvent<string>) {
    const closedId = event.detail;
    // Add to closed set (NOT removed from entries store, so calendar still shows it)
    closedEntryIds.update(s => { const next = new Set(s); next.add(closedId); return next; });
    // If the closed entry was currently selected, check dirty then clear the editor
    if (currentEntryIdVal === closedId) {
      if (!await checkDirty()) {
        // User cancelled - undo the close
        closedEntryIds.update(s => { const next = new Set(s); next.delete(closedId); return next; });
        return;
      }
      clearUndoState();
      skipDirtyTracking = true;
      currentEntryId.set(null);
      currentEntry.set(null);
      editorContent.set('');
      entryTitle.set('');
      translations.set({});
      explanation.set(null);
      isDirty.set(false);
      skipDirtyTracking = false;
    }
  }

  async function handleDeleteCurrent() {
    if (!currentEntryIdVal) return;
    const confirmed = await confirm(get(t)('dialog.deleteMessage'), {
      title: get(t)('dialog.deleteTitle'),
      kind: 'warning',
    });
    if (!confirmed) return;
    try {
      await invoke('delete_entry', { id: currentEntryIdVal });
      await loadEntries();
      skipDirtyTracking = true;
      currentEntryId.set(null);
      currentEntry.set(null);
      editorContent.set('');
      entryTitle.set('');
      translations.set({});
      explanation.set(null);
      isDirty.set(false);
      skipDirtyTracking = false;
    } catch (e: any) {
      error.set(get(t)('error.deleteFailed', { values: { detail: e.toString() } }));
    }
  }

  function isSectionHeader(line: string): boolean {
    if (line.length > 60 || line.length === 0) return false;
    const headerPatterns = [
      /ver\./i, /version/i, /版本/, /バージョン/,
      /^english/i, /^中文/, /^日本語/, /^한국어/, /^italiano/i,
      /^french/i, /^german/i, /^spanish/i, /^português/i,
      /^翻[譯訳]/i, /^translation/i, /^original/i,
    ];
    return headerPatterns.some(p => p.test(line));
  }

  function applyPartialCorrection(original: string, corrected: string): string {
    // Always check for section headers first
    const lines = original.split('\n');
    let sectionStart = -1;

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i].trim();
      if (line.length === 0) continue;
      if (isSectionHeader(line)) {
        // Include preceding blank lines
        let start = i;
        while (start > 0 && lines[start - 1].trim() === '') {
          start--;
        }
        sectionStart = start;
        break;
      }
    }

    if (sectionStart === -1) {
      // No section headers found, replace everything
      return corrected;
    }

    // Replace everything before the first section header, preserve the rest
    const preservedContent = lines.slice(sectionStart).join('\n');
    return corrected.trimEnd() + '\n\n' + preservedContent;
  }

  function handleApplyCorrection() {
    const langKey = selectedLangsVal[0] || configVal.default_language;
    const corrected = translationsVal[langKey];
    if (!corrected) return;

    // Save undo state
    let prevExplanation: string | null = null;
    explanation.subscribe(v => prevExplanation = v)();

    undoState = {
      previousContent: editorVal,
      previousTranslations: { ...translationsVal },
      previousExplanation: prevExplanation,
      countdown: 5,
    };

    // Apply correction
    skipDirtyTracking = true;
    const newContent = applyPartialCorrection(editorVal, corrected);
    editorContent.set(newContent);
    translations.set({});
    explanation.set(null);
    skipDirtyTracking = false;
    isDirty.set(true);

    // Start countdown
    if (undoTimer) clearInterval(undoTimer);
    undoTimer = setInterval(() => {
      if (undoState) {
        undoState.countdown -= 1;
        if (undoState.countdown <= 0) {
          clearInterval(undoTimer!);
          undoTimer = null;
          undoState = null;
        } else {
          // Trigger reactivity
          undoState = { ...undoState };
        }
      }
    }, 1000);
  }

  function handleUndo() {
    if (!undoState) return;
    if (undoTimer) {
      clearInterval(undoTimer);
      undoTimer = null;
    }
    skipDirtyTracking = true;
    editorContent.set(undoState.previousContent);
    translations.set(undoState.previousTranslations);
    explanation.set(undoState.previousExplanation);
    skipDirtyTracking = false;
    isDirty.set(true);
    undoState = null;
  }

  async function handleQuickTranslate(event: CustomEvent<{text: string, targetLanguage: string}>) {
    const { text, targetLanguage } = event.detail;
    try {
      // Wrap the text so AI treats it as content to translate, not an instruction
      const wrappedText = `[Text to translate]\n\n${text}`;
      const results: Record<string, string> = await invoke('translate_text', {
        text: wrappedText,
        targetLanguages: [targetLanguage],
      });
      const translated = results[targetLanguage] || '';
      if (editorRef) {
        editorRef.setQuickTranslation(translated);
      }
    } catch (e: any) {
      if (editorRef) {
        editorRef.setQuickTranslation(get(t)('error.translationFailed', { values: { detail: e.toString() } }));
      }
    }
  }

  function handlePrint() {
    showPrintDialog = true;
  }

  async function executePrint(mode: 'original' | 'full') {
    showPrintDialog = false;
    printMode = mode;
    // Give Svelte a tick to update the DOM before printing
    await new Promise(resolve => setTimeout(resolve, 50));
    try {
      await invoke('print_page');
    } catch (e: any) {
      error.set(get(t)('error.printFailed', { values: { detail: e.toString() } }));
    } finally {
      printMode = null;
    }
  }
</script>

<div class="app-layout">
  <!-- Sidebar -->
  <aside class="sidebar">
    <div class="sidebar-header">
      <h1 class="app-title">{$t('app.title')}</h1>
      <div class="sidebar-actions">
        <ThemeSwitcher />
        <button class="icon-btn" onclick={() => showSettings.set(true)} title={$t('app.settings')}>
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
          </svg>
        </button>
      </div>
    </div>

    <Calendar
      on:dateSelect={handleDateSelect}
      on:monthChange={handleMonthChange}
      on:entrySelect={handleEntrySelect}
    />

    <EntryList on:select={handleEntrySelect} on:close={handleEntryClose} />
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
            <button class="btn btn-new-entry" onclick={handleNewEntry} title={$t('app.newEntryTitle')}>
              {$t('app.newEntry')}
            </button>
          </div>
          <div class="title-row">
            <input
              type="text"
              class="title-input"
              placeholder={$t('app.titlePlaceholder')}
              value={titleVal}
              oninput={(e) => entryTitle.set((e.target as HTMLInputElement).value)}
            />
            <input
              type="date"
              class="date-input"
              value={dateVal}
              oninput={(e) => handleDateInputChange((e.target as HTMLInputElement).value)}
            />
          </div>
          {#if currentEntryVal?.meta?.created_at || currentEntryVal?.meta?.updated_at}
            <div class="timestamps">
              {#if currentEntryVal.meta.created_at}
                <span>{$t('app.created')} {formatTimestamp(currentEntryVal.meta.created_at)}</span>
              {/if}
              {#if currentEntryVal.meta.updated_at}
                <span>{$t('app.edited')} {formatTimestamp(currentEntryVal.meta.updated_at)}</span>
              {/if}
            </div>
          {/if}
        </div>

        <Editor bind:this={editorRef} on:quickTranslate={handleQuickTranslate} />

        <div class="editor-footer">
          {#if errorVal}
            <div class="error-msg">{errorVal}</div>
          {/if}
          <div class="action-buttons">
            {#if currentEntryIdVal}
              <button class="btn btn-danger" onclick={handleDeleteCurrent} title={$t('app.deleteEntryTitle')}>
                🗑
              </button>
            {/if}
            <button class="btn btn-primary" onclick={handleSubmit} disabled={processingVal}>
              {#if processingVal}
                {$t('app.processing')}
              {:else if modeVal === 'correction'}
                {$t('app.submitCorrection')}
              {:else}
                {$t('app.submitTranslation')}
              {/if}
            </button>
            <button class="btn btn-secondary" onclick={handleSave}>
              {$t('app.save')}
            </button>
            <button class="btn btn-outline" onclick={handlePrint} title={$t('app.exportPdf')}>
              {$t('app.exportPdf')}
            </button>
          </div>
        </div>
      </div>
    {/if}
  </main>

  <!-- Result panel -->
  <aside class="result-panel">
    {#if undoState && undoState.countdown > 0}
      <div class="undo-panel">
        <div class="undo-message">{$t('app.correctionApplied')}</div>
        <button class="btn btn-undo" onclick={handleUndo}>
          {$t('app.undo', { values: { countdown: undoState.countdown } })}
        </button>
      </div>
    {:else if modeVal === 'correction'}
      <Correction on:apply={handleApplyCorrection} />
    {:else}
      <Translation />
    {/if}

    {#if Object.keys(translationsVal).length > 0 && !undoState}
      <div class="panel-footer">
        <button class="btn btn-outline no-print" onclick={handlePrint}>
          {$t('app.exportPdf')}
        </button>
      </div>
    {/if}
  </aside>
</div>

<!-- Print-only view: completely separate from the app layout -->
{#if printMode}
<div class="print-view">
  <div class="print-header">
    <h1>{titleVal || `${dateVal} ${$t('app.defaultDiaryTitle')}`}</h1>
    <div class="print-date">{dateVal}</div>
  </div>

  <div class="print-section">
    <div class="print-section-label">{$t('print.original')}</div>
    <div class="print-text">{editorVal}</div>
  </div>

  {#if printMode === 'full'}
    {#if modeVal === 'correction'}
      {#if translationsVal[selectedLangsVal[0] || configVal?.default_language]}
        <div class="print-section">
          <div class="print-section-label">{$t('print.corrected')}</div>
          <div class="print-text">{translationsVal[selectedLangsVal[0] || configVal?.default_language]}</div>
        </div>
      {/if}
    {:else}
      {#each Object.entries(translationsVal) as [langCode, text]}
        <div class="print-section">
          <div class="print-section-label">{configVal?.languages?.find(l => l.code === langCode)?.name || langCode}</div>
          <div class="print-text">{text}</div>
        </div>
      {/each}
    {/if}
  {/if}
</div>
{/if}

<!-- Print dialog modal -->
{#if showPrintDialog}
<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div class="print-dialog-overlay" onclick={() => showPrintDialog = false}>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="print-dialog" onclick={(e) => e.stopPropagation()} role="dialog" aria-modal="true" aria-label="Export PDF options" tabindex="-1">
    <h2 class="print-dialog-title">{$t('print.title')}</h2>
    <p class="print-dialog-desc">{$t('print.description')}</p>
    <div class="print-dialog-buttons">
      <button class="btn btn-primary" onclick={() => executePrint('original')}>
        {$t('print.originalOnly')}
      </button>
      <button class="btn btn-secondary" onclick={() => executePrint('full')} disabled={Object.keys(translationsVal).length === 0}>
        {$t('print.originalAndAi')}
      </button>
      {#if Object.keys(translationsVal).length === 0}
        <p class="print-dialog-note">{$t('print.noAiResults')}</p>
      {/if}
      <button class="btn btn-outline" onclick={() => showPrintDialog = false}>
        {$t('print.cancel')}
      </button>
    </div>
  </div>
</div>
{/if}

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

  .title-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .title-row .title-input {
    flex: 1;
  }

  .date-input {
    font-size: 13px;
    padding: 4px 8px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--bg-main);
    color: var(--text-secondary);
    cursor: pointer;
  }

  .timestamps {
    display: flex;
    gap: 16px;
    font-size: 11px;
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

  .btn-danger {
    color: var(--diff-removed-text);
    border: 1px solid transparent;
    padding: 8px 10px;
    font-size: 15px;
    transition: all 0.15s;
  }

  .btn-danger:hover {
    background: var(--diff-removed-bg);
    border-color: var(--diff-removed-text);
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

  .undo-panel {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 20px;
    height: 100%;
    padding: 40px 24px;
  }

  .undo-message {
    font-size: 14px;
    color: var(--text-secondary);
    text-align: center;
  }

  .btn-undo {
    padding: 12px 28px;
    font-size: 16px;
    font-weight: 600;
    border-radius: var(--radius);
    background: var(--diff-removed-bg);
    color: var(--diff-removed-text);
    border: 2px solid var(--diff-removed-text);
    transition: all 0.15s;
    min-width: 140px;
  }

  .btn-undo:hover {
    opacity: 0.85;
  }

  .print-dialog-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
  }

  .print-dialog {
    background: var(--bg-main);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 28px 32px;
    min-width: 300px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  }

  .print-dialog-title {
    font-size: 16px;
    font-weight: 600;
  }

  .print-dialog-desc {
    font-size: 13px;
    color: var(--text-secondary);
    margin-top: -8px;
  }

  .print-dialog-buttons {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .print-dialog-buttons .btn {
    width: 100%;
    text-align: center;
  }

  .btn-secondary:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .print-dialog-note {
    font-size: 12px;
    color: var(--text-muted);
    text-align: center;
    margin-top: -4px;
  }
</style>
