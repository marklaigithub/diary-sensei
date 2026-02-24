<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { config, showSettings } from './store';
  import type { AppConfig, LanguageConfig, AiProvider } from './types';

  let configVal: AppConfig;
  let saving = $state(false);
  let saveMsg = $state('');

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
      saveMsg = 'Saved!';
      setTimeout(() => saveMsg = '', 2000);
    } catch (e: any) {
      saveMsg = 'Error: ' + e.toString();
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
    <h2>Settings</h2>
    <button class="close-btn" onclick={close}>
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
    </button>
  </div>

  <div class="settings-body">
    <div class="setting-group">
      <label class="setting-label">AI Provider</label>
      <select class="setting-input" bind:value={aiProvider}>
        <option value="ollama">Local (Ollama) — free, private</option>
        <option value="claude">Claude API — best quality, paid</option>
      </select>
      <p class="setting-hint">
        {#if aiProvider === 'ollama'}
          Runs locally on your machine. No data leaves your computer.
        {:else}
          Requires API key. Data sent to Anthropic servers.
        {/if}
      </p>
    </div>

    {#if aiProvider === 'ollama'}
      <div class="setting-group">
        <label class="setting-label">Ollama Model</label>
        <select class="setting-input" bind:value={ollamaModel}>
          <option value="gemma2:9b">Gemma 2 9B (Google, recommended)</option>
          <option value="gemma2:2b">Gemma 2 2B (Google, faster)</option>
          <option value="llama3.1:8b">Llama 3.1 8B (Meta)</option>
          <option value="mistral:7b">Mistral 7B (Mistral AI)</option>
          <option value="phi3:medium">Phi-3 Medium (Microsoft)</option>
        </select>
        <p class="setting-hint">Install with: ollama pull {ollamaModel}</p>
      </div>

      <div class="setting-group">
        <label class="setting-label">Ollama URL</label>
        <input
          type="text"
          class="setting-input"
          bind:value={ollamaUrl}
          placeholder="http://localhost:11434"
        />
      </div>
    {:else}
      <div class="setting-group">
        <label class="setting-label">Claude API Key</label>
        <input
          type="password"
          class="setting-input"
          bind:value={apiKey}
          placeholder="sk-ant-..."
        />
        <p class="setting-hint">Get your API key from console.anthropic.com</p>
      </div>
    {/if}

    <div class="setting-group">
      <label class="setting-label">Default Language</label>
      <select class="setting-input" bind:value={defaultLang}>
        {#each languages as lang}
          <option value={lang.code}>{lang.name} ({lang.code})</option>
        {/each}
      </select>
    </div>

    <div class="setting-group">
      <label class="setting-label">Entries Directory</label>
      <input
        type="text"
        class="setting-input"
        bind:value={entriesDir}
        placeholder="~/Documents/diary-language/entries"
      />
    </div>

    <div class="setting-group">
      <label class="setting-label">Global Date Format (optional)</label>
      <input
        type="text"
        class="setting-input"
        bind:value={globalDateFormat}
        placeholder="Use language default"
      />
      <p class="setting-hint">e.g. YYYY年MM月DD日（ddd）, MMM DD, YYYY (ddd)</p>
    </div>

    <div class="setting-group">
      <label class="setting-label">Languages</label>
      <div class="languages-list">
        {#each languages as lang, idx}
          {#if editingLangIdx === idx}
            <div class="lang-edit-form">
              <input type="text" class="lang-input" bind:value={editForm.code} placeholder="Code (e.g. ko)" />
              <input type="text" class="lang-input lang-input-wide" bind:value={editForm.name} placeholder="Name (e.g. Korean)" />
              <input type="text" class="lang-input lang-input-wide" bind:value={editForm.date_format} placeholder="Date format" />
              <div class="lang-edit-actions">
                <button class="lang-action-btn save" onclick={saveEditLang} title="Save">✓</button>
                <button class="lang-action-btn cancel" onclick={cancelEditLang} title="Cancel">✕</button>
              </div>
            </div>
          {:else}
            <div class="lang-item">
              <span class="lang-code">{lang.code}</span>
              <span class="lang-name">{lang.name}</span>
              <span class="lang-format">{lang.date_format}</span>
              <div class="lang-item-actions">
                <button class="lang-action-btn" onclick={() => startEditLang(idx)} title="Edit">✎</button>
                <button class="lang-action-btn delete" onclick={() => deleteLang(idx)} title="Delete">✕</button>
              </div>
            </div>
          {/if}
        {/each}
        {#if addingLang}
          <div class="lang-edit-form">
            <input type="text" class="lang-input" bind:value={editForm.code} placeholder="Code (e.g. ko)" />
            <input type="text" class="lang-input lang-input-wide" bind:value={editForm.name} placeholder="Name (e.g. Korean)" />
            <input type="text" class="lang-input lang-input-wide" bind:value={editForm.date_format} placeholder="Date format" />
            <div class="lang-edit-actions">
              <button class="lang-action-btn save" onclick={saveEditLang} title="Save">✓</button>
              <button class="lang-action-btn cancel" onclick={cancelEditLang} title="Cancel">✕</button>
            </div>
          </div>
        {:else}
          <button class="add-lang-btn" onclick={startAddLang}>+ Add Language</button>
        {/if}
      </div>
    </div>
  </div>

  <div class="settings-footer">
    {#if saveMsg}
      <span class="save-msg" class:error={saveMsg.startsWith('Error')}>{saveMsg}</span>
    {/if}
    <button class="btn btn-secondary" onclick={close}>Cancel</button>
    <button class="btn btn-primary" onclick={handleSave} disabled={saving}>
      {saving ? 'Saving...' : 'Save'}
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
