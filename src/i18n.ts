import { register, init } from 'svelte-i18n';

register('en', () => import('./locales/en.json'));
register('zh-TW', () => import('./locales/zh-TW.json'));
register('ja', () => import('./locales/ja.json'));
register('ko', () => import('./locales/ko.json'));
register('it', () => import('./locales/it.json'));

init({
  fallbackLocale: 'en',
  initialLocale: localStorage.getItem('diary-sensei-locale') || 'en',
});
