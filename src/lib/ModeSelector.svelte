<script lang="ts">
  import { appMode, config, selectedTargetLanguages } from './store';
  import type { AppMode, AppConfig } from './types';

  let mode: AppMode;
  let configVal: AppConfig;
  let selectedLangs: string[];

  appMode.subscribe(v => mode = v);
  config.subscribe(v => configVal = v);
  selectedTargetLanguages.subscribe(v => selectedLangs = v);

  function setMode(m: AppMode) {
    appMode.set(m);
    // When switching to correction, default to single language
    if (m === 'correction') {
      selectedTargetLanguages.set([configVal.default_language]);
    }
  }

  function toggleLanguage(code: string) {
    if (mode === 'correction') {
      // Correction mode: single language only
      selectedTargetLanguages.set([code]);
    } else {
      // Translation mode: toggle in/out
      const current = [...selectedLangs];
      const idx = current.indexOf(code);
      if (idx >= 0 && current.length > 1) {
        current.splice(idx, 1);
      } else if (idx < 0) {
        current.push(code);
      }
      selectedTargetLanguages.set(current);
    }
  }
</script>

<div class="mode-selector">
  <div class="mode-toggle">
    <button
      class="mode-btn"
      class:active={mode === 'correction'}
      onclick={() => setMode('correction')}
    >
      ✍️ Writing
    </button>
    <button
      class="mode-btn"
      class:active={mode === 'translation'}
      onclick={() => setMode('translation')}
    >
      🔄 Translation
    </button>
  </div>

  <div class="lang-picker">
    {#each configVal?.languages || [] as lang}
      <button
        class="lang-chip"
        class:selected={selectedLangs.includes(lang.code)}
        onclick={() => toggleLanguage(lang.code)}
      >
        {lang.code.toUpperCase()}
      </button>
    {/each}
  </div>
</div>

<style>
  .mode-selector {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .mode-toggle {
    display: flex;
    gap: 4px;
    background: var(--bg-hover);
    padding: 3px;
    border-radius: var(--radius);
  }

  .mode-btn {
    padding: 6px 12px;
    border-radius: calc(var(--radius) - 2px);
    font-size: 13px;
    font-weight: 500;
    color: var(--text-secondary);
    transition: all 0.2s;
    flex: 1;
    text-align: center;
  }

  .mode-btn:hover {
    color: var(--text-primary);
  }

  .mode-btn.active {
    background: var(--bg-panel);
    color: var(--text-primary);
    box-shadow: 0 1px 3px var(--shadow);
  }

  .lang-picker {
    display: flex;
    gap: 4px;
    margin-top: 2px;
    flex-wrap: wrap;
  }

  .lang-chip {
    padding: 3px 10px;
    border-radius: 12px;
    font-size: 11px;
    font-weight: 600;
    border: 1px solid var(--border);
    color: var(--text-secondary);
    transition: all 0.15s;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .lang-chip:hover {
    border-color: var(--accent);
    color: var(--text-primary);
  }

  .lang-chip.selected {
    background: var(--accent);
    color: var(--btn-primary-text);
    border-color: var(--accent);
  }
</style>
