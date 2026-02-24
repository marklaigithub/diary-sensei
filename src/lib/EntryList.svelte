<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { t } from 'svelte-i18n';
  import { entries, currentEntryId, searchQuery, searchResults, closedEntryIds } from './store';
  import type { EntryListItem } from './types';

  const dispatch = createEventDispatcher();

  let entryList: EntryListItem[] = [];
  let currentIdVal: string | null;
  let query: string = '';
  let searchResultsVal: EntryListItem[] | null = null;
  let searchTimeout: ReturnType<typeof setTimeout> | null = null;
  let closedIds: Set<string> = new Set();

  entries.subscribe(v => entryList = v);
  currentEntryId.subscribe(v => currentIdVal = v);
  searchQuery.subscribe(v => query = v);
  searchResults.subscribe(v => searchResultsVal = v);
  closedEntryIds.subscribe(v => closedIds = v);

  function modeIcon(mode: string): string {
    return mode === 'correction' ? '✍️' : '🔄';
  }

  function selectEntry(id: string) {
    dispatch('select', id);
  }

  function formatShortDate(date: string): string {
    const parts = date.split('-');
    const monthIdx = parseInt(parts[1]) - 1;
    return `${$t('calendar.months.' + monthIdx)} ${parseInt(parts[2])}`;
  }

  function formatTime(id: string): string {
    // id format: "2026-02-24_143052"
    const timePart = id.split('_')[1];
    if (!timePart || timePart.length < 4) return '';
    return `${timePart.slice(0, 2)}:${timePart.slice(2, 4)}`;
  }

  function handleSearchInput(e: Event) {
    const value = (e.target as HTMLInputElement).value;
    searchQuery.set(value);

    if (searchTimeout) clearTimeout(searchTimeout);

    if (!value.trim()) {
      searchResults.set(null);
      return;
    }

    // Debounce search by 300ms
    searchTimeout = setTimeout(async () => {
      try {
        const results: EntryListItem[] = await invoke('search_entries', { query: value });
        searchResults.set(results);
      } catch (err) {
        console.error('Search failed:', err);
      }
    }, 300);
  }

  function handleClose(e: Event, entry: EntryListItem) {
    e.stopPropagation();
    dispatch('close', entry.id);
  }

  function clearSearch() {
    if (searchTimeout) clearTimeout(searchTimeout);
    searchQuery.set('');
    searchResults.set(null);
  }

  $: displayList = searchResultsVal !== null ? searchResultsVal : entryList.filter(e => !closedIds.has(e.id));
  $: isSearching = searchResultsVal !== null;
</script>

<div class="entry-list">
  <div class="search-bar">
    <input
      type="text"
      class="search-input"
      placeholder={$t('entryList.searchPlaceholder')}
      value={query}
      oninput={handleSearchInput}
    />
    {#if query}
      <button class="clear-btn" onclick={clearSearch} title={$t('entryList.clearSearch')}>✕</button>
    {/if}
  </div>

  <div class="list-header">
    <span>{isSearching ? $t('entryList.results') : $t('entryList.entries')}</span>
    <span class="count">{displayList.length}</span>
  </div>

  {#if displayList.length === 0}
    <div class="empty-state">
      {isSearching ? $t('entryList.noMatchingEntries') : $t('entryList.noEntriesThisMonth')}
    </div>
  {:else}
    {#each displayList as entry}
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="entry-item"
        class:active={entry.id === currentIdVal}
        onclick={() => selectEntry(entry.id)}
        onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); selectEntry(entry.id); } }}
        role="button"
        tabindex="0"
      >
        <div class="entry-date">
          <span class="mode-icon">{modeIcon(entry.mode)}</span>
          <span>{formatShortDate(entry.date)} {formatTime(entry.id)}</span>
          <button
            class="delete-btn"
            onclick={(e) => handleClose(e, entry)}
            title={$t('entryList.close')}
          >✕</button>
        </div>
        <div class="entry-title">{entry.title}</div>
      </div>
    {/each}
  {/if}
</div>

<style>
  .entry-list {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
  }

  .search-bar {
    position: relative;
    padding: 0 4px 8px;
  }

  .search-input {
    width: 100%;
    padding: 7px 28px 7px 10px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--bg-main);
    font-size: 13px;
    color: var(--text-primary);
    outline: none;
    transition: border-color 0.2s;
  }

  .search-input:focus {
    border-color: var(--accent);
  }

  .search-input::placeholder {
    color: var(--text-muted);
  }

  .clear-btn {
    position: absolute;
    right: 10px;
    top: 50%;
    transform: translateY(calc(-50% - 4px));
    font-size: 12px;
    color: var(--text-muted);
    padding: 2px 4px;
    border-radius: 2px;
    line-height: 1;
  }

  .clear-btn:hover {
    color: var(--text-primary);
    background: var(--bg-hover);
  }

  .list-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px;
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .count {
    background: var(--bg-hover);
    padding: 1px 6px;
    border-radius: 10px;
    font-size: 11px;
  }

  .empty-state {
    text-align: center;
    color: var(--text-muted);
    padding: 24px 16px;
    font-size: 13px;
  }

  .entry-item {
    display: block;
    width: 100%;
    text-align: left;
    padding: 10px 12px;
    border-radius: var(--radius);
    margin-bottom: 2px;
    transition: all 0.15s;
    cursor: pointer;
  }

  .entry-item:hover {
    background: var(--bg-hover);
  }

  .entry-item.active {
    background: var(--bg-active);
  }

  .entry-date {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: var(--text-secondary);
    margin-bottom: 2px;
  }

  .delete-btn {
    margin-left: auto;
    font-size: 11px;
    color: var(--text-muted);
    padding: 1px 4px;
    border-radius: 3px;
    line-height: 1;
    opacity: 0;
    transition: all 0.15s;
  }

  .entry-item:hover .delete-btn {
    opacity: 1;
  }

  .delete-btn:hover {
    color: var(--text-primary);
    background: var(--bg-hover);
  }

  .mode-icon {
    font-size: 14px;
  }

  .entry-title {
    font-size: 13px;
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
