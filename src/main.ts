import './styles.css';
import './i18n';
import { waitLocale } from 'svelte-i18n';
import App from './App.svelte';
import { mount } from 'svelte';

// Set default theme
if (!document.documentElement.dataset.theme) {
  document.documentElement.dataset.theme = 'warm-light';
}

// Wait for locale to load before mounting — svelte-i18n uses async import()
waitLocale().then(() => {
  mount(App, { target: document.getElementById('app')! });
});
