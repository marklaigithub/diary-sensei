import { writable } from 'svelte/store';
import type { AppConfig, DiaryEntry, EntryListItem, AppMode } from './types';

export const currentDate = writable<string>(new Date().toISOString().slice(0, 10));
export const currentEntryId = writable<string | null>(null);
export const selectedYear = writable<number>(new Date().getFullYear());
export const selectedMonth = writable<number>(new Date().getMonth() + 1);
export const entries = writable<EntryListItem[]>([]);
export const currentEntry = writable<DiaryEntry | null>(null);
export const appMode = writable<AppMode>('correction');
export const isLoading = writable<boolean>(false);
export const error = writable<string>('');
export const config = writable<AppConfig>({
  api_key: '',
  default_language: 'ja',
  theme: 'warm-light',
  languages: [
    { code: 'ja', name: '日本語', date_format: 'YYYY年MM月DD日（ddd）' },
    { code: 'en', name: 'English', date_format: 'MMM DD, YYYY (ddd)' },
    { code: 'it', name: 'Italiano', date_format: 'DD/MM/YYYY (ddd)' },
  ],
  entries_dir: '',
  global_date_format: null,
  ai_provider: 'ollama',
  ollama_model: 'gemma2:9b',
  ollama_url: 'http://localhost:11434',
});
export const showSettings = writable<boolean>(false);
export const editorContent = writable<string>('');
export const entryTitle = writable<string>('');
export const translations = writable<Record<string, string>>({});
export const selectedTargetLanguages = writable<string[]>(['ja']);
export const isProcessing = writable<boolean>(false);
export const searchQuery = writable<string>('');
export const searchResults = writable<EntryListItem[] | null>(null);
