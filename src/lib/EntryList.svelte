<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { entries, currentEntryId } from './store';
  import type { EntryListItem } from './types';

  const dispatch = createEventDispatcher();

  let entryList: EntryListItem[] = [];
  let currentIdVal: string | null;

  entries.subscribe(v => entryList = v);
  currentEntryId.subscribe(v => currentIdVal = v);

  function modeIcon(mode: string): string {
    return mode === 'correction' ? '✍️' : '🔄';
  }

  function selectEntry(id: string) {
    dispatch('select', id);
  }

  function formatShortDate(date: string): string {
    const parts = date.split('-');
    const months = ['Jan','Feb','Mar','Apr','May','Jun','Jul','Aug','Sep','Oct','Nov','Dec'];
    return `${months[parseInt(parts[1]) - 1]} ${parseInt(parts[2])}`;
  }

  function formatTime(id: string): string {
    // id format: "2026-02-24_143052"
    const timePart = id.split('_')[1];
    if (!timePart || timePart.length < 4) return '';
    return `${timePart.slice(0, 2)}:${timePart.slice(2, 4)}`;
  }
</script>

<div class="entry-list">
  <div class="list-header">
    <span>Entries</span>
    <span class="count">{entryList.length}</span>
  </div>

  {#if entryList.length === 0}
    <div class="empty-state">No entries this month</div>
  {:else}
    {#each entryList as entry}
      <button
        class="entry-item"
        class:active={entry.id === currentIdVal}
        onclick={() => selectEntry(entry.id)}
      >
        <div class="entry-date">
          <span class="mode-icon">{modeIcon(entry.mode)}</span>
          <span>{formatShortDate(entry.date)} {formatTime(entry.id)}</span>
        </div>
        <div class="entry-title">{entry.title}</div>
      </button>
    {/each}
  {/if}
</div>

<style>
  .entry-list {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
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
