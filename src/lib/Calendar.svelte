<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { selectedYear, selectedMonth, entries, currentDate } from './store';
  import type { EntryListItem } from './types';

  const dispatch = createEventDispatcher();

  let year: number;
  let month: number;
  let entryDates: Set<string> = new Set();
  let currentDateVal: string;

  selectedYear.subscribe(v => year = v);
  selectedMonth.subscribe(v => month = v);
  currentDate.subscribe(v => currentDateVal = v);
  entries.subscribe(items => {
    entryDates = new Set(items.map(e => e.date));
  });

  const WEEKDAYS = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];

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

  function isToday(day: number): boolean {
    const today = new Date().toISOString().slice(0, 10);
    return formatDate(year, month, day) === today;
  }

  function isSelected(day: number): boolean {
    return formatDate(year, month, day) === currentDateVal;
  }

  function hasEntry(day: number): boolean {
    return entryDates.has(formatDate(year, month, day));
  }

  $: calendarDays = getCalendarDays(year, month);
  $: monthLabel = `${year}年${month}月`;
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
    {#each WEEKDAYS as day}
      <div class="weekday">{day}</div>
    {/each}

    {#each calendarDays as day}
      {#if day === null}
        <div class="day empty"></div>
      {:else}
        <button
          class="day"
          class:today={isToday(day)}
          class:selected={isSelected(day)}
          class:has-entry={hasEntry(day)}
          onclick={() => selectDate(day)}
        >
          {day}
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

  .day.has-entry::after {
    content: '';
    position: absolute;
    bottom: 2px;
    left: 50%;
    transform: translateX(-50%);
    width: 4px;
    height: 4px;
    border-radius: 50%;
    background: var(--calendar-has-entry);
  }

  .day.selected.has-entry::after {
    background: var(--btn-primary-text);
  }

  .day.empty {
    cursor: default;
  }
</style>
