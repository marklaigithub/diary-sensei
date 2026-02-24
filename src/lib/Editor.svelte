<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { editorContent, appMode, config } from './store';
  import type { AppConfig } from './types';

  const dispatch = createEventDispatcher();

  let content: string = '';
  let modeVal: string;
  let configVal: AppConfig;

  editorContent.subscribe(v => content = v);
  appMode.subscribe(v => modeVal = v);
  config.subscribe(v => configVal = v);

  // Context menu state
  let showContextMenu = false;
  let contextMenuX = 0;
  let contextMenuY = 0;
  let selectedText = '';

  // Translation bubble state
  let showBubble = false;
  let bubbleX = 0;
  let bubbleY = 0;
  let bubbleText = '';
  let bubbleLang = '';
  let bubbleLoading = false;

  function handleInput(e: Event) {
    const textarea = e.target as HTMLTextAreaElement;
    editorContent.set(textarea.value);
  }

  function getPlaceholder(): string {
    if (modeVal === 'correction') {
      return 'Write your diary in the target language...';
    }
    return '用繁體中文寫日記，AI 會翻譯成目標語言...';
  }

  function handleContextMenu(e: MouseEvent) {
    const textarea = e.target as HTMLTextAreaElement;
    const selection = textarea.value.substring(textarea.selectionStart, textarea.selectionEnd);

    if (!selection.trim()) {
      // No selection — let default context menu show
      return;
    }

    e.preventDefault();
    selectedText = selection;
    showContextMenu = true;
    showBubble = false;

    // Position context menu at mouse location
    const editorRect = textarea.closest('.editor-wrapper')?.getBoundingClientRect();
    if (editorRect) {
      contextMenuX = e.clientX - editorRect.left;
      contextMenuY = e.clientY - editorRect.top;
    }
  }

  function handleTranslateSelect(langCode: string, langName: string) {
    showContextMenu = false;
    bubbleLang = langName;
    bubbleText = '';
    bubbleLoading = true;
    showBubble = true;
    bubbleX = contextMenuX;
    bubbleY = contextMenuY;

    dispatch('quickTranslate', {
      text: selectedText,
      targetLanguage: langCode,
    });
  }

  export function setQuickTranslation(text: string) {
    bubbleText = text;
    bubbleLoading = false;
  }

  function copyBubbleText() {
    navigator.clipboard.writeText(bubbleText);
  }

  function closeBubble() {
    showBubble = false;
    bubbleText = '';
  }

  function closeContextMenu() {
    showContextMenu = false;
  }

  function handleDocClick(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (showContextMenu && !target.closest('.context-menu')) {
      showContextMenu = false;
    }
    if (showBubble && !target.closest('.translate-bubble')) {
      closeBubble();
    }
  }
</script>

<svelte:window onclick={handleDocClick} />

<div class="editor-wrapper">
  <textarea
    class="editor-textarea"
    value={content}
    oninput={handleInput}
    oncontextmenu={handleContextMenu}
    placeholder={getPlaceholder()}
    spellcheck="false"
  ></textarea>

  {#if showContextMenu}
    <div class="context-menu" style="left: {contextMenuX}px; top: {contextMenuY}px;">
      <div class="context-menu-header">Translate to...</div>
      {#each configVal?.languages || [] as lang}
        <button
          class="context-menu-item"
          onclick={() => handleTranslateSelect(lang.code, lang.name)}
        >
          {lang.name}
        </button>
      {/each}
    </div>
  {/if}

  {#if showBubble}
    <div class="translate-bubble" style="left: {bubbleX}px; top: {bubbleY + 24}px;">
      <div class="bubble-header">
        <span class="bubble-lang">{bubbleLang}</span>
        {#if bubbleText}
          <button class="bubble-copy" onclick={copyBubbleText} title="Copy">📋</button>
        {/if}
        <button class="bubble-close" onclick={closeBubble}>✕</button>
      </div>
      <div class="bubble-content">
        {#if bubbleLoading}
          <div class="bubble-loading">
            <span class="bubble-spinner"></span>
            Translating...
          </div>
        {:else}
          {bubbleText}
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .editor-wrapper {
    flex: 1;
    overflow: hidden;
    display: flex;
    position: relative;
  }

  .editor-textarea {
    width: 100%;
    height: 100%;
    border: none;
    background: transparent;
    resize: none;
    padding: 16px;
    line-height: 1.8;
    font-size: 15px;
    outline: none;
    color: var(--text-primary);
    font-family: inherit;
  }

  .editor-textarea::placeholder {
    color: var(--text-muted);
  }

  /* Context menu */
  .context-menu {
    position: absolute;
    background: var(--bg-panel);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    min-width: 160px;
    z-index: 100;
    overflow: hidden;
  }

  .context-menu-header {
    padding: 8px 12px;
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    border-bottom: 1px solid var(--border-light);
  }

  .context-menu-item {
    display: block;
    width: 100%;
    text-align: left;
    padding: 8px 12px;
    font-size: 13px;
    color: var(--text-primary);
    transition: background 0.1s;
  }

  .context-menu-item:hover {
    background: var(--bg-hover);
  }

  /* Translation bubble */
  .translate-bubble {
    position: absolute;
    background: var(--bg-panel);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
    min-width: 200px;
    max-width: 360px;
    z-index: 100;
  }

  .bubble-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    border-bottom: 1px solid var(--border-light);
  }

  .bubble-lang {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    flex: 1;
  }

  .bubble-copy {
    font-size: 13px;
    padding: 2px 4px;
    border-radius: 2px;
    opacity: 0.6;
    transition: opacity 0.15s;
  }

  .bubble-copy:hover {
    opacity: 1;
  }

  .bubble-close {
    font-size: 12px;
    padding: 2px 4px;
    color: var(--text-muted);
    border-radius: 2px;
  }

  .bubble-close:hover {
    color: var(--text-primary);
    background: var(--bg-hover);
  }

  .bubble-content {
    padding: 12px;
    font-size: 14px;
    line-height: 1.7;
    white-space: pre-wrap;
    max-height: 200px;
    overflow-y: auto;
  }

  .bubble-loading {
    display: flex;
    align-items: center;
    gap: 8px;
    color: var(--text-secondary);
    font-size: 13px;
  }

  .bubble-spinner {
    display: inline-block;
    width: 14px;
    height: 14px;
    border: 2px solid var(--border);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
