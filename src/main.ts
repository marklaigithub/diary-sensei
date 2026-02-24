import './styles.css';
import App from './App.svelte';
import { mount } from 'svelte';

// Set default theme
if (!document.documentElement.dataset.theme) {
  document.documentElement.dataset.theme = 'warm-light';
}

const app = mount(App, { target: document.getElementById('app')! });

export default app;
