import { renderLanding } from './pages/landing';
import { renderInstall } from './pages/install';
import { renderDocs } from './pages/docs';
import { renderPlayground, initPlayground, destroyPlayground } from './pages/playground';

type Route = 'home' | 'install' | 'docs' | 'playground';

function getRoute(): Route {
  const hash = window.location.hash.replace('#/', '').replace('#', '');
  switch (hash) {
    case 'install': return 'install';
    case 'docs': return 'docs';
    case 'playground': return 'playground';
    default: return 'home';
  }
}

function render(route: Route) {
  const page = document.getElementById('page')!;

  // Cleanup previous playground if leaving it
  destroyPlayground();

  // Update active nav link
  document.querySelectorAll('.nav-links a').forEach(a => {
    a.classList.remove('active');
    const href = a.getAttribute('href') || '';
    const linkRoute = href.replace('#/', '').replace('#', '') || 'home';
    if (linkRoute === route || (route === 'home' && (linkRoute === '' || linkRoute === '#'))) {
      a.classList.add('active');
    }
  });

  switch (route) {
    case 'home':
      page.innerHTML = renderLanding();
      page.className = 'page fade-in';
      setupLandingEvents();
      break;
    case 'install':
      page.innerHTML = renderInstall();
      page.className = 'page fade-in';
      setupCopyEvents();
      break;
    case 'docs':
      page.innerHTML = renderDocs();
      page.className = 'page fade-in';
      setupCopyEvents();
      break;
    case 'playground':
      page.innerHTML = renderPlayground();
      page.className = 'page';
      initPlayground();
      break;
  }

  // Scroll to top unless playground
  if (route !== 'playground') {
    window.scrollTo(0, 0);
  }
}

function setupLandingEvents() {
  // Copy install command
  const installCmd = document.querySelector('.install-command');
  if (installCmd) {
    installCmd.addEventListener('click', () => {
      navigator.clipboard.writeText('npm install -g goonsharp').then(() => {
        const icon = installCmd.querySelector('.copy-icon');
        if (icon) {
          icon.textContent = '✓';
          setTimeout(() => { icon.textContent = '📋'; }, 1500);
        }
      });
    });
  }
}

function setupCopyEvents() {
  document.querySelectorAll('.code-block[data-copy]').forEach(block => {
    block.addEventListener('click', () => {
      const text = (block as HTMLElement).dataset.copy || block.textContent || '';
      navigator.clipboard.writeText(text.trim()).then(() => {
        block.classList.add('copied');
        setTimeout(() => block.classList.remove('copied'), 1500);
      });
    });
  });
}

export function initRouter() {
  window.addEventListener('hashchange', () => render(getRoute()));
  render(getRoute());
}

export function navigate(route: string) {
  window.location.hash = `#/${route}`;
}
