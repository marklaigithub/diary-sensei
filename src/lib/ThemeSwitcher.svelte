<script lang="ts">
  import { config } from './store';
  import type { AppConfig } from './types';

  const themes = [
    { id: 'warm-light', label: 'Warm', icon: '🌅' },
    { id: 'cool-light', label: 'Cool', icon: '🌊' },
    { id: 'dark', label: 'Dark', icon: '🌙' },
  ];

  let showDropdown = $state(false);
  let configVal: AppConfig;
  config.subscribe(v => configVal = v);

  function selectTheme(themeId: string) {
    document.documentElement.dataset.theme = themeId;
    config.update(c => ({ ...c, theme: themeId }));
    showDropdown = false;
  }

  function getCurrentIcon(): string {
    return themes.find(t => t.id === configVal?.theme)?.icon || '🎨';
  }
</script>

<div class="theme-switcher">
  <button class="icon-btn" onclick={() => showDropdown = !showDropdown} title="Theme">
    {getCurrentIcon()}
  </button>

  {#if showDropdown}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="dropdown-overlay" onclick={() => showDropdown = false} onkeydown={() => {}}></div>
    <div class="dropdown">
      {#each themes as theme}
        <button
          class="dropdown-item"
          class:active={configVal?.theme === theme.id}
          onclick={() => selectTheme(theme.id)}
        >
          <span>{theme.icon}</span>
          <span>{theme.label}</span>
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
