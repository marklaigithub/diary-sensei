<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { t, locale } from 'svelte-i18n';
  import { config, showSettings } from './store';
  import type { AppConfig, LanguageConfig, AiProvider } from './types';

  let configVal: AppConfig;
  let saving = $state(false);
  let saveMsg = $state('');
  let saveIsError = $state(false);

  config.subscribe(v => configVal = v);

  let aiProvider = $state<AiProvider>('ollama');
  let apiKey = $state('');
  let ollamaModel = $state('');
  let ollamaUrl = $state('');
  let defaultLang = $state('');
  let entriesDir = $state('');
  let globalDateFormat = $state('');
  let languages = $state<LanguageConfig[]>([]);
  let editingLangIdx = $state<number | null>(null);
  let addingLang = $state(false);
  let editForm = $state({ code: '', name: '', date_format: '' });
  let uiLocale = $state(localStorage.getItem('diary-sensei-locale') || 'en');

  const uiLanguageOptions = [
    { code: 'en', label: 'English' },
    { code: 'zh-TW', label: '繁體中文' },
    { code: 'ja', label: '日本語' },
    { code: 'ko', label: '한국어' },
    { code: 'it', label: 'Italiano' },
  ];

  $effect(() => {
    if (configVal) {
      aiProvider = configVal.ai_provider || 'ollama';
      apiKey = configVal.api_key || '';
      ollamaModel = configVal.ollama_model || 'gemma2:9b';
      ollamaUrl = configVal.ollama_url || 'http://localhost:11434';
      defaultLang = configVal.default_language || 'ja';
      entriesDir = configVal.entries_dir || '';
      globalDateFormat = configVal.global_date_format || '';
      languages = (configVal.languages || []).map(l => ({ ...l }));
    }
  });

  function handleLocaleChange(e: Event) {
    const newLocale = (e.target as HTMLSelectElement).value;
    uiLocale = newLocale;
    locale.set(newLocale);
    localStorage.setItem('diary-sensei-locale', newLocale);
  }

  function startEditLang(idx: number) {
    editingLangIdx = idx;
    editForm = { ...languages[idx] };
  }

  function cancelEditLang() {
    editingLangIdx = null;
    addingLang = false;
    editForm = { code: '', name: '', date_format: '' };
  }

  function saveEditLang() {
    if (!editForm.code.trim() || !editForm.name.trim()) return;
    if (addingLang) {
      languages = [...languages, { ...editForm }];
      addingLang = false;
    } else if (editingLangIdx !== null) {
      languages = languages.map((l, i) => i === editingLangIdx ? { ...editForm } : l);
      editingLangIdx = null;
    }
    editForm = { code: '', name: '', date_format: '' };
  }

  function deleteLang(idx: number) {
    const deleted = languages[idx];
    languages = languages.filter((_, i) => i !== idx);
    if (deleted.code === defaultLang) {
      defaultLang = languages.length > 0 ? languages[0].code : '';
    }
  }

  function startAddLang() {
    addingLang = true;
    editingLangIdx = null;
    editForm = { code: '', name: '', date_format: '' };
  }

  async function handleSave() {
    saving = true;
    saveMsg = '';
    saveIsError = false;
    try {
      const updated: AppConfig = {
        ...configVal,
        ai_provider: aiProvider,
        api_key: apiKey,
        ollama_model: ollamaModel,
        ollama_url: ollamaUrl,
        default_language: defaultLang,
        entries_dir: entriesDir,
        global_date_format: globalDateFormat || null,
        languages: languages,
      };
      await invoke('save_config', { config: updated });
      config.set(updated);
      saveMsg = $t('settings.saved');
      setTimeout(() => saveMsg = '', 2000);
    } catch (e: any) {
      saveMsg = $t('settings.saveFailed', { values: { detail: e.toString() } });
      saveIsError = true;
    } finally {
      saving = false;
    }
  }

  function close() {
    showSettings.set(false);
  }
</script>

<div class="settings">
  <div class="settings-header">
    <h2>{$t('settings.title')}</h2>
    <button class="close-btn" onclick={close}>
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
    </button>
  </div>

  <div class="settings-body">
    <div class="setting-group">
      <label class="setting-label">{$t('settings.uiLanguage')}</label>
      <select class="setting-input" value={uiLocale} onchange={handleLocaleChange}>
        {#each uiLanguageOptions as opt}
          <option value={opt.code}>{opt.label}</option>
        {/each}
      </select>
    </div>

    <div class="setting-group">
      <label class="setting-label">{$t('settings.aiProvider')}</label>
      <select class="setting-input" bind:value={aiProvider}>
        <option value="ollama">{$t('settings.ollamaLocal')}</option>
        <option value="claude">{$t('settings.claudeApi')}</option>
      </select>
      <p class="setting-hint">
        {#if aiProvider === 'ollama'}
          {$t('settings.ollamaHint')}
        {:else}
          {$t('settings.claudeHint')}
        {/if}
      </p>
    </div>

    {#if aiProvider === 'ollama'}
      <div class="setting-group">
        <label class="setting-label">{$t('settings.ollamaModel')}</label>
        <select class="setting-input" bind:value={ollamaModel}>
          <option value="gemma2:9b">{$t('settings.ollamaGemma2_9b')}</option>
          <option value="gemma2:2b">{$t('settings.ollamaGemma2_2b')}</option>
          <option value="llama3.1:8b">{$t('settings.ollamaLlama3')}</option>
          <option value="mistral:7b">{$t('settings.ollamaMistral')}</option>
          <option value="phi3:medium">{$t('settings.ollamaPhi3')}</option>
        </select>
        <p class="setting-hint">{$t('settings.ollamaInstallHint', { values: { model: ollamaModel } })}</p>
      </div>

      <div class="setting-group">
        <label class="setting-label">{$t('settings.ollamaUrl')}</label>
        <input
          type="text"
          class="setting-input"
          bind:value={ollamaUrl}
          placeholder="http://localhost:11434"
        />
      </div>
    {:else}
      <div class="setting-group">
        <label class="setting-label">{$t('settings.claudeApiKey')}</label>
        <input
          type="password"
          class="setting-input"
          bind:value={apiKey}
          placeholder="sk-ant-..."
        />
        <p class="setting-hint">{$t('settings.claudeApiKeyHint')}</p>
      </div>
    {/if}

    <div class="setting-group">
      <label class="setting-label">{$t('settings.defaultLanguage')}</label>
      <select class="setting-input" bind:value={defaultLang}>
        {#each languages as lang}
          <option value={lang.code}>{lang.name} ({lang.code})</option>
        {/each}
      </select>
    </div>

    <div class="setting-group">
      <label class="setting-label">{$t('settings.entriesDir')}</label>
      <input
        type="text"
        class="setting-input"
        bind:value={entriesDir}
        placeholder="~/Documents/diary-language/entries"
      />
    </div>

    <div class="setting-group">
      <label class="setting-label">{$t('settings.globalDateFormat')}</label>
      <input
        type="text"
        class="setting-input"
        bind:value={globalDateFormat}
        placeholder={$t('settings.globalDateFormatPlaceholder')}
      />
      <p class="setting-hint">{$t('settings.globalDateFormatHint')}</p>
    </div>

    <div class="setting-group">
      <label class="setting-label">{$t('settings.languages')}</label>
      <div class="languages-list">
        {#each languages as lang, idx}
          {#if editingLangIdx === idx}
            <div class="lang-edit-form">
              <input type="text" class="lang-input" bind:value={editForm.code} placeholder={$t('settings.langCodePlaceholder')} />
              <input type="text" class="lang-input lang-input-wide" bind:value={editForm.name} placeholder={$t('settings.langNamePlaceholder')} />
              <input type="text" class="lang-input lang-input-wide" bind:value={editForm.date_format} placeholder={$t('settings.langDateFormatPlaceholder')} />
              <div class="lang-edit-actions">
                <button class="lang-action-btn save" onclick={saveEditLang} title={$t('settings.save')}>✓</button>
                <button class="lang-action-btn cancel" onclick={cancelEditLang} title={$t('settings.cancel')}>✕</button>
              </div>
            </div>
          {:else}
            <div class="lang-item">
              <span class="lang-code">{lang.code}</span>
              <span class="lang-name">{lang.name}</span>
              <span class="lang-format">{lang.date_format}</span>
              <div class="lang-item-actions">
                <button class="lang-action-btn" onclick={() => startEditLang(idx)} title={$t('settings.editLanguage')}>✎</button>
                <button class="lang-action-btn delete" onclick={() => deleteLang(idx)} title={$t('settings.deleteLanguage')}>✕</button>
              </div>
            </div>
          {/if}
        {/each}
        {#if addingLang}
          <div class="lang-edit-form">
            <input type="text" class="lang-input" bind:value={editForm.code} placeholder={$t('settings.langCodePlaceholder')} />
            <input type="text" class="lang-input lang-input-wide" bind:value={editForm.name} placeholder={$t('settings.langNamePlaceholder')} />
            <input type="text" class="lang-input lang-input-wide" bind:value={editForm.date_format} placeholder={$t('settings.langDateFormatPlaceholder')} />
            <div class="lang-edit-actions">
              <button class="lang-action-btn save" onclick={saveEditLang} title={$t('settings.save')}>✓</button>
              <button class="lang-action-btn cancel" onclick={cancelEditLang} title={$t('settings.cancel')}>✕</button>
            </div>
          </div>
        {:else}
          <button class="add-lang-btn" onclick={startAddLang}>{$t('settings.addLanguage')}</button>
        {/if}
      </div>
    </div>
  </div>

  <div class="settings-footer">
    {#if saveMsg}
      <span class="save-msg" class:error={saveIsError}>{saveMsg}</span>
    {/if}
    <button class="btn btn-secondary" onclick={close}>{$t('settings.cancel')}</button>
    <button class="btn btn-primary" onclick={handleSave} disabled={saving}>
      {saving ? $t('settings.saving') : $t('settings.save')}
    </button>
  </div>
</div>

<style>
  .settings {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--bg-main);
  }

  .settings-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px;
    border-bottom: 1px solid var(--border);
  }

  .settings-header h2 {
    font-size: 16px;
    font-weight: 600;
  }

  .close-btn {
    padding: 4px;
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
  }

  .close-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .settings-body {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
  }

  .setting-group {
    margin-bottom: 20px;
  }

  .setting-label {
    display: block;
    font-size: 13px;
    font-weight: 600;
    margin-bottom: 6px;
    color: var(--text-primary);
  }

  .setting-input {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid var(--input-border);
    border-radius: var(--radius-sm);
    background: var(--input-bg);
    color: var(--text-primary);
    font-size: 14px;
  }

  .setting-hint {
    font-size: 12px;
    color: var(--text-muted);
    margin-top: 4px;
  }

  .languages-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .lang-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 12px;
    background: var(--bg-panel);
    border: 1px solid var(--border-light);
    border-radius: var(--radius-sm);
    font-size: 13px;
  }

  .lang-code {
    font-weight: 600;
    color: var(--accent);
    min-width: 24px;
  }

  .lang-name {
    flex: 1;
  }

  .lang-format {
    color: var(--text-muted);
    font-size: 12px;
  }

  .lang-item-actions {
    display: flex;
    gap: 4px;
    opacity: 0;
    transition: opacity 0.15s;
  }

  .lang-item:hover .lang-item-actions {
    opacity: 1;
  }

  .lang-action-btn {
    font-size: 13px;
    padding: 2px 6px;
    border-radius: 3px;
    color: var(--text-muted);
    transition: all 0.15s;
  }

  .lang-action-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .lang-action-btn.save {
    color: var(--diff-added-text);
  }

  .lang-action-btn.cancel,
  .lang-action-btn.delete {
    color: var(--diff-removed-text);
  }

  .lang-edit-form {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    background: var(--bg-panel);
    border: 1px solid var(--accent);
    border-radius: var(--radius-sm);
  }

  .lang-input {
    padding: 4px 8px;
    border: 1px solid var(--input-border);
    border-radius: var(--radius-sm);
    background: var(--input-bg);
    color: var(--text-primary);
    font-size: 13px;
    width: 60px;
  }

  .lang-input-wide {
    flex: 1;
    min-width: 100px;
  }

  .lang-edit-actions {
    display: flex;
    gap: 4px;
  }

  .add-lang-btn {
    width: 100%;
    padding: 8px;
    border: 1px dashed var(--border);
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    font-size: 13px;
    transition: all 0.15s;
  }

  .add-lang-btn:hover {
    border-color: var(--accent);
    color: var(--accent);
    background: var(--bg-hover);
  }

  .settings-footer {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 8px;
    padding: 12px 16px;
    border-top: 1px solid var(--border);
  }

  .save-msg {
    margin-right: auto;
    font-size: 13px;
    color: var(--diff-added-text);
  }

  .save-msg.error {
    color: var(--diff-removed-text);
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
  }

  .btn-secondary {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .btn-secondary:hover {
    background: var(--bg-active);
  }
</style>
