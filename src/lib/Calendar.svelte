<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { t } from 'svelte-i18n';
  import { selectedYear, selectedMonth, entries, currentDate } from './store';
  import type { EntryListItem } from './types';

  const dispatch = createEventDispatcher();

  let year: number;
  let month: number;
  let entryMap: Map<string, EntryListItem[]> = new Map();
  let currentDateVal: string;

  selectedYear.subscribe(v => year = v);
  selectedMonth.subscribe(v => month = v);
  currentDate.subscribe(v => currentDateVal = v);
  entries.subscribe(items => {
    const map = new Map<string, EntryListItem[]>();
    for (const item of items) {
      const existing = map.get(item.date) || [];
      existing.push(item);
      map.set(item.date, existing);
    }
    entryMap = map;
  });

  function getDaysInMonth(y: number, m: number): number {
    return new Date(y, m, 0).getDate();
  }

  function getFirstDayOfWeek(y: number, m: number): number {
    const day = new Date(y, m - 1, 1).getDay();
    return day === 0 ? 6 : day - 1; // Monday = 0
  }

  function getCalendarDays(y: number, m: number) {
    const daysInMonth = getDaysInMonth(y, m);
    const firstDay = getFirstDayOfWeek(y, m);
    const days: (number | null)[] = [];

    for (let i = 0; i < firstDay; i++) days.push(null);
    for (let d = 1; d <= daysInMonth; d++) days.push(d);

    return days;
  }

  function formatDate(y: number, m: number, d: number): string {
    return `${y}-${String(m).padStart(2, '0')}-${String(d).padStart(2, '0')}`;
  }

  function prevMonth() {
    if (month === 1) {
      dispatch('monthChange', { year: year - 1, month: 12 });
    } else {
      dispatch('monthChange', { year, month: month - 1 });
    }
  }

  function nextMonth() {
    if (month === 12) {
      dispatch('monthChange', { year: year + 1, month: 1 });
    } else {
      dispatch('monthChange', { year, month: month + 1 });
    }
  }

  function selectDate(day: number) {
    const date = formatDate(year, month, day);
    dispatch('dateSelect', date);
  }

  function selectEntry(id: string) {
    dispatch('entrySelect', id);
  }

  function isToday(day: number): boolean {
    const today = new Date().toISOString().slice(0, 10);
    return formatDate(year, month, day) === today;
  }

  function isSelected(day: number): boolean {
    return formatDate(year, month, day) === currentDateVal;
  }

  $: calendarData = getCalendarDays(year, month).map(day => {
    if (day === null) return { day: null as number | null, count: 0, dayEntries: [] as EntryListItem[] };
    const dateStr = formatDate(year, month, day);
    const dayEntries = entryMap.get(dateStr) || [];
    return { day: day as number | null, count: dayEntries.length, dayEntries };
  });
  $: monthName = $t('calendar.months.' + (month - 1));
  $: monthLabel = $t('calendar.monthFormat', { values: { year, month: monthName } });
  $: weekdays = Array.from({ length: 7 }, (_, i) => $t('calendar.weekdays.' + i));
</script>

<div class="calendar">
  <div class="calendar-nav">
    <button class="nav-btn" onclick={prevMonth}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
    </button>
    <span class="month-label">{monthLabel}</span>
    <button class="nav-btn" onclick={nextMonth}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="9 18 15 12 9 6"/></svg>
    </button>
  </div>

  <div class="calendar-grid">
    {#each weekdays as day}
      <div class="weekday">{day}</div>
    {/each}

    {#each calendarData as { day, count, dayEntries }}
      {#if day === null}
        <div class="day empty"></div>
      {:else}
        <button
          class="day"
          class:today={isToday(day)}
          class:selected={isSelected(day)}
          class:has-entry={count > 0}
          onclick={() => selectDate(day)}
        >
          {day}
          {#if count === 1}
            <span class="entry-dot"></span>
          {:else if count > 1}
            <span class="entry-count">{count}</span>
          {/if}
          {#if count >= 1}
            <div class="day-tooltip">
              {#each dayEntries.slice(0, 5) as entry}
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <div class="tooltip-item" onclick={(e) => { e.stopPropagation(); selectEntry(entry.id); }}>{entry.title}</div>
              {/each}
              {#if count > 5}
                <div class="tooltip-more">{$t('calendar.more', { values: { count: count - 5 } })}</div>
              {/if}
            </div>
          {/if}
        </button>
      {/if}
    {/each}
  </div>
</div>

<style>
  .calendar {
    padding: 12px;
  }

  .calendar-nav {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 12px;
  }

  .nav-btn {
    padding: 4px;
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
  }

  .nav-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .month-label {
    font-weight: 600;
    font-size: 14px;
  }

  .calendar-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 2px;
  }

  .weekday {
    text-align: center;
    font-size: 11px;
    color: var(--text-muted);
    padding: 4px 0;
    font-weight: 500;
  }

  .day {
    text-align: center;
    padding: 6px 2px;
    font-size: 12px;
    border-radius: var(--radius-sm);
    position: relative;
    transition: all 0.15s;
    overflow: visible;
  }

  .day:not(.empty):hover {
    background: var(--calendar-hover);
  }

  .day.today {
    background: var(--calendar-today);
    font-weight: 600;
  }

  .day.selected {
    background: var(--accent);
    color: var(--btn-primary-text);
    font-weight: 600;
  }

  /* Entry dot for single entry */
  .entry-dot {
    position: absolute;
    bottom: 2px;
    left: 50%;
    transform: translateX(-50%);
    width: 4px;
    height: 4px;
    border-radius: 50%;
    background: var(--calendar-has-entry);
    display: block;
  }

  .day.selected .entry-dot {
    background: var(--btn-primary-text);
  }

  /* Entry count badge for multiple entries */
  .entry-count {
    position: absolute;
    bottom: 1px;
    right: 1px;
    min-width: 13px;
    height: 13px;
    padding: 0 2px;
    font-size: 9px;
    font-weight: 600;
    line-height: 13px;
    text-align: center;
    border-radius: 6px;
    background: var(--calendar-has-entry);
    color: var(--btn-primary-text);
    display: block;
  }

  .day.selected .entry-count {
    background: var(--btn-primary-text);
    color: var(--accent);
  }

  /* Tooltip */
  .day-tooltip {
    position: absolute;
    top: calc(100% + 4px);
    left: 50%;
    transform: translateX(-50%);
    z-index: 10;
    background: var(--bg-panel);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    box-shadow: 0 4px 12px var(--shadow);
    padding: 6px 8px;
    min-width: 120px;
    max-width: 180px;
    white-space: nowrap;
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.15s ease;
  }

  .day:hover .day-tooltip {
    opacity: 1;
    pointer-events: auto;
  }

  .day-tooltip::before {
    content: '';
    position: absolute;
    top: -8px;
    left: 0;
    right: 0;
    height: 8px;
  }

  .tooltip-item {
    font-size: 11px;
    color: var(--text-primary);
    padding: 2px 4px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    text-align: left;
    border-radius: 3px;
    cursor: pointer;
    transition: background 0.1s;
  }

  .tooltip-item:hover {
    background: var(--bg-hover);
  }

  .tooltip-item:not(:last-child) {
    border-bottom: none;
  }

  .tooltip-more {
    font-size: 10px;
    color: var(--text-muted);
    padding-top: 3px;
    text-align: center;
  }

  .day.empty {
    cursor: default;
  }
</style>
