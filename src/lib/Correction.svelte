<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { diffChars } from 'diff';
  import type { Change } from 'diff';
  import { editorContent, translations, isProcessing, selectedTargetLanguages, explanation } from './store';

  const dispatch = createEventDispatcher();

  let original: string = '';
  let corrected: string = '';
  let processing: boolean;
  let selectedLangs: string[] = [];
  let explanationText: string | null = null;

  editorContent.subscribe(v => original = v);
  selectedTargetLanguages.subscribe(v => selectedLangs = v);
  translations.subscribe(v => {
    const lang = selectedLangs[0];
    corrected = lang ? (v[lang] || '') : '';
  });
  isProcessing.subscribe(v => processing = v);
  explanation.subscribe(v => explanationText = v);

  function computeDiff(orig: string, corr: string): Change[] {
    if (!orig || !corr) return [];
    return diffChars(orig, corr);
  }

  $: diffParts = computeDiff(original, corrected);
  $: hasChanges = diffParts.some(p => p.added || p.removed);
</script>

<div class="correction-panel">
  <div class="panel-header">
    <h3>Correction</h3>
    {#if corrected && !hasChanges}
      <span class="badge perfect">Perfect!</span>
    {/if}
  </div>

  <div class="panel-content">
    {#if processing}
      <div class="loading">
        <div class="spinner"></div>
        <span>AI is reviewing...</span>
      </div>
    {:else if corrected}
      <div class="diff-display">
        {#each diffParts as part}
          {#if part.added}
            <span class="diff-added">{part.value}</span>
          {:else if part.removed}
            <span class="diff-removed">{part.value}</span>
          {:else}
            <span>{part.value}</span>
          {/if}
        {/each}
      </div>

      <div class="corrected-full">
        <h4>Corrected version</h4>
        <div class="text-block">{corrected}</div>
      </div>

      {#if explanationText}
        <div class="explanation-section">
          <h4>Why these changes?</h4>
          <div class="explanation-text">{explanationText}</div>
        </div>
      {/if}

      <div class="action-bar">
        <button class="btn btn-accept" onclick={() => dispatch('apply')}>
          ✓ Accept Correction
        </button>
      </div>
    {:else}
      <div class="placeholder-msg">
        Submit your writing to see AI corrections here.
      </div>
    {/if}
  </div>
</div>

<style>
  .correction-panel {
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

  .badge {
    font-size: 11px;
    padding: 2px 8px;
    border-radius: 10px;
    font-weight: 500;
  }

  .badge.perfect {
    background: var(--diff-added-bg);
    color: var(--diff-added-text);
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

  .diff-display {
    line-height: 1.8;
    font-size: 15px;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .corrected-full {
    border-top: 1px solid var(--border-light);
    padding-top: 16px;
  }

  .corrected-full h4 {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 8px;
  }

  .text-block {
    line-height: 1.8;
    font-size: 15px;
    white-space: pre-wrap;
  }

  .placeholder-msg {
    color: var(--text-muted);
    text-align: center;
    padding: 40px 20px;
    font-size: 14px;
  }

  .explanation-section {
    border-top: 1px solid var(--border-light);
    padding-top: 16px;
  }

  .explanation-section h4 {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 8px;
  }

  .explanation-text {
    font-size: 13px;
    line-height: 1.7;
    color: var(--text-secondary);
    white-space: pre-wrap;
  }

  .action-bar {
    padding: 4px 0 0;
    border-top: 1px solid var(--border-light);
  }

  .btn-accept {
    width: 100%;
    padding: 8px;
    background: var(--diff-added-bg);
    color: var(--diff-added-text);
    border-radius: var(--radius);
    font-weight: 500;
    transition: all 0.2s;
  }

  .btn-accept:hover {
    opacity: 0.8;
  }
</style>
