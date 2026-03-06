import './style.css';
import { initRouter } from './router';
import { renderNav } from './components/nav';

// Mount navigation
const app = document.getElementById('app')!;
app.innerHTML = renderNav() + '<main id="page" class="page"></main>';

// Initialize theme from localStorage
const savedTheme = localStorage.getItem('goon-theme') || 'dark';
document.documentElement.setAttribute('data-theme', savedTheme);

// Initialize router
initRouter();

// Setup theme toggle
document.addEventListener('click', (e) => {
  const target = e.target as HTMLElement;
  if (target.closest('.theme-toggle')) {
    const html = document.documentElement;
    const current = html.getAttribute('data-theme');
    const next = current === 'dark' ? 'light' : 'dark';
    html.setAttribute('data-theme', next);
    localStorage.setItem('goon-theme', next);
    // Update toggle icon
    const btn = document.querySelector('.theme-toggle');
    if (btn) btn.textContent = next === 'dark' ? '☀️' : '🌙';
    // Update meta theme-color
    const meta = document.querySelector('meta[name="theme-color"]');
    if (meta) meta.setAttribute('content', next === 'dark' ? '#080510' : '#fdf6ff');
  }
});

// Setup hamburger menu
document.addEventListener('click', (e) => {
  const target = e.target as HTMLElement;
  if (target.closest('.nav-hamburger')) {
    document.querySelector('.nav-links')?.classList.toggle('open');
  }
});

// Close mobile menu on nav click
document.addEventListener('click', (e) => {
  const target = e.target as HTMLElement;
  if (target.closest('.nav-links a')) {
    document.querySelector('.nav-links')?.classList.remove('open');
  }
});
