<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { t } from 'svelte-i18n';
  import { config } from './store';
  import type { AppConfig } from './types';

  const themeIds = [
    { id: 'warm-light', icon: '🌅', key: 'warm' },
    { id: 'cool-light', icon: '🌊', key: 'cool' },
    { id: 'dark', icon: '🌙', key: 'dark' },
  ];

  let showDropdown = $state(false);
  let configVal: AppConfig;
  config.subscribe(v => configVal = v);

  function selectTheme(themeId: string) {
    document.documentElement.dataset.theme = themeId;
    const updated = { ...configVal, theme: themeId };
    config.set(updated);
    showDropdown = false;
    // Persist theme choice (fire-and-forget)
    invoke('save_config', { config: updated }).catch(console.error);
  }

  function getCurrentIcon(): string {
    return themeIds.find(t => t.id === configVal?.theme)?.icon || '🎨';
  }
</script>

<div class="theme-switcher">
  <button class="icon-btn" onclick={() => showDropdown = !showDropdown} title={$t('theme.label')}>
    {getCurrentIcon()}
  </button>

  {#if showDropdown}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="dropdown-overlay" onclick={() => showDropdown = false} onkeydown={() => {}}></div>
    <div class="dropdown">
      {#each themeIds as theme}
        <button
          class="dropdown-item"
          class:active={configVal?.theme === theme.id}
          onclick={() => selectTheme(theme.id)}
        >
          <span>{theme.icon}</span>
          <span>{$t('theme.' + theme.key)}</span>
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .theme-switcher {
    position: relative;
  }

  .icon-btn {
    padding: 6px;
    border-radius: var(--radius-sm);
    font-size: 16px;
    line-height: 1;
  }

  .icon-btn:hover {
    background: var(--bg-hover);
  }

  .dropdown-overlay {
    position: fixed;
    inset: 0;
    z-index: 99;
  }

  .dropdown {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 4px;
    background: var(--bg-panel);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    box-shadow: 0 4px 12px var(--shadow);
    z-index: 100;
    min-width: 120px;
    padding: 4px;
  }

  .dropdown-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 8px 12px;
    border-radius: var(--radius-sm);
    font-size: 13px;
    transition: background 0.15s;
  }

  .dropdown-item:hover {
    background: var(--bg-hover);
  }

  .dropdown-item.active {
    background: var(--bg-active);
    font-weight: 500;
  }
</style>
