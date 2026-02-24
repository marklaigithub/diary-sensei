export interface EntryMeta {
  id: string;              // "2026-02-24_143052"
  date: string;
  title: string;
  mode: 'correction' | 'translation';
  languages: string[];     // target languages, e.g. ["ja", "en"]
  date_format?: string;
}

export interface DiaryEntry {
  meta: EntryMeta;
  original: string;
  translations: Record<string, string>;  // lang_code -> result text
}

export interface EntryListItem {
  id: string;
  date: string;
  title: string;
  mode: string;
  languages: string[];
}

export interface LanguageConfig {
  code: string;
  name: string;
  date_format: string;
}

export type AiProvider = 'ollama' | 'claude';

export interface AppConfig {
  api_key: string;
  default_language: string;
  theme: string;
  languages: LanguageConfig[];
  entries_dir: string;
  global_date_format: string | null;
  ai_provider: AiProvider;
  ollama_model: string;
  ollama_url: string;
}

export type AppMode = 'correction' | 'translation';
export type ViewMode = 'edit' | 'read';
