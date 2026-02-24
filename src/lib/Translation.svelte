<script lang="ts">
  import { editorContent, translations, isProcessing, config, selectedTargetLanguages } from './store';
  import type { AppConfig } from './types';

  let original: string = '';
  let translationsVal: Record<string, string> = {};
  let processing: boolean;
  let configVal: AppConfig;
  let selectedLangs: string[] = [];

  editorContent.subscribe(v => original = v);
  translations.subscribe(v => translationsVal = v);
  isProcessing.subscribe(v => processing = v);
  config.subscribe(v => configVal = v);
  selectedTargetLanguages.subscribe(v => selectedLangs = v);

  function getLanguageName(code: string): string {
    const lang = configVal?.languages.find(l => l.code === code);
    return lang?.name || code;
  }

  function copyToClipboard(text: string) {
    navigator.clipboard.writeText(text);
  }

  $: hasTranslations = Object.keys(translationsVal).length > 0;
  $: translationEntries = Object.entries(translationsVal);
</script>

<div class="translation-panel">
  <div class="panel-header">
    <h3>Translation</h3>
    {#if hasTranslations}
      <span class="lang-count">{translationEntries.length} language{translationEntries.length > 1 ? 's' : ''}</span>
    {/if}
  </div>

  <div class="panel-content">
    {#if processing}
      <div class="loading">
        <div class="spinner"></div>
        <span>Translating {selectedLangs.length} language{selectedLangs.length > 1 ? 's' : ''}...</span>
      </div>
    {:else if hasTranslations}
      <!-- Original text section -->
      <div class="section original-section">
        <div class="section-label">Original</div>
        <div class="section-text">{original}</div>
      </div>

      <!-- Each translation -->
      {#each translationEntries as [langCode, text]}
        <div class="section translated-section">
          <div class="section-header">
            <div class="section-label">{getLanguageName(langCode)}</div>
            <button class="copy-btn" onclick={() => copyToClipboard(text)} title="Copy">
              📋
            </button>
          </div>
          <div class="section-text">{text}</div>
        </div>
      {/each}
    {:else}
      <div class="placeholder-msg">
        Select target languages and submit to see translations here.
      </div>
    {/if}
  </div>
</div>

<style>
  .translation-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px;
    border-bottom: 1px solid var(--border-light);
  }

  .panel-header h3 {
    font-size: 14px;
    font-weight: 600;
  }

  .lang-count {
    font-size: 11px;
    color: var(--text-muted);
    background: var(--bg-hover);
    padding: 2px 8px;
    border-radius: 10px;
  }

  .panel-content {
    flex: 1;
    padding: 16px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .loading {
    display: flex;
    align-items: center;
    gap: 8px;
    color: var(--text-secondary);
    padding: 24px;
    justify-content: center;
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid var(--border);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .section {
    border: 1px solid var(--border);
    border-radius: var(--radius);
    overflow: hidden;
  }

  .section-label {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    padding: 8px 12px;
    background: var(--bg-sidebar);
    border-bottom: 1px solid var(--border-light);
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: var(--bg-sidebar);
    border-bottom: 1px solid var(--border-light);
  }

  .section-text {
    padding: 12px;
    line-height: 1.8;
    font-size: 15px;
    white-space: pre-wrap;
  }

  .copy-btn {
    padding: 4px 8px;
    font-size: 14px;
    border-radius: var(--radius-sm);
    opacity: 0.6;
    transition: opacity 0.15s;
  }

  .copy-btn:hover {
    opacity: 1;
  }

  .placeholder-msg {
    color: var(--text-muted);
    text-align: center;
    padding: 40px 20px;
    font-size: 14px;
  }
</style>
