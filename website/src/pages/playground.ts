// WASM module reference
let wasmModule: any = null;
let wasmLoading = false;
let wasmError: string | null = null;

const EXAMPLES: Record<string, { name: string; code: string }> = {
  hello: {
    name: 'Hello Goon',
    code: `// Hello Goon — The simplest GoonSharp program
goonsesh main() {
    goonprint!("Hello, Goon World! 🟣");
}`,
  },
  fizzbuzz: {
    name: 'FizzBuzz',
    code: `// FizzBuzz but make it goon
goonsesh main() {
    goonfor i goonin 1..=20 {
        goonif (i % 15 == 0) {
            goonprint!("GoonBuzz");
        } goonnah goonif (i % 3 == 0) {
            goonprint!("Goon");
        } goonnah goonif (i % 5 == 0) {
            goonprint!("Buzz");
        } goonnah {
            goonprint!("{}", i);
        }
    }
}`,
  },
  structs: {
    name: 'Structs',
    code: `// Structs in GoonSharp
goonstruct Player {
    name: GoonString,
    score: i69,
}

goonimpl Player {
    goonsesh new(name: GoonString) -> Player {
        Player { name, score: 0 }
    }

    goonsesh level_up(&mut self) {
        self.score = self.score + 100;
    }
}

goonsesh main() {
    goonlet mut p = Player::new("xX_G00N_Xx".to_string());
    p.level_up();
    goonprint!("{} — score: {}", p.name, p.score);
}`,
  },
  enums: {
    name: 'Enums & Match',
    code: `// Enums and pattern matching
goonenum Direction {
    North,
    South,
    East,
    West,
}

goonsesh describe(d: Direction) -> GoonString {
    goonmatch d {
        Direction::North => "Going up! ⬆️".to_string(),
        Direction::South => "Going down! ⬇️".to_string(),
        Direction::East => "Going right! ➡️".to_string(),
        Direction::West => "Going left! ⬅️".to_string(),
    }
}

goonsesh main() {
    goonlet d = Direction::North;
    goonprint!("{}", describe(d));
}`,
  },
  closures: {
    name: 'Closures & Iterators',
    code: `// Closures and iterator chains
goonsesh main() {
    goonlet nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Filter evens and double them
    goonlet result: Vec<i64> = nums.iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 2)
        .collect();

    goonprint!("Doubled evens: {:?}", result);

    // Sum with fold
    goonlet sum: i64 = nums.iter().fold(0, |acc, x| acc + x);
    goonprint!("Sum: {}", sum);
}`,
  },
  error: {
    name: 'Error Handling',
    code: `// Error handling with Result
goonsesh divide(a: f64, b: f64) -> Result<f64, GoonString> {
    goonif (b == 0.0) {
        Err("Division by zero!".to_string())
    } goonnah {
        Ok(a / b)
    }
}

goonsesh main() {
    goonmatch divide(420.0, 69.0) {
        Ok(result) => goonprint!("420 / 69 = {:.2}", result),
        Err(e) => goonprint!("Error: {}", e),
    }

    goonmatch divide(1.0, 0.0) {
        Ok(result) => goonprint!("Result: {}", result),
        Err(e) => goonprint!("Caught: {}", e),
    }
}`,
  },
};

async function loadWasm(): Promise<void> {
  if (wasmModule) return;
  if (wasmLoading) return;

  wasmLoading = true;
  wasmError = null;

  try {
    // Dynamically load the WASM JS module from public/wasm/
    const baseUrl = import.meta.url;
    const jsUrl = new URL('/wasm/goonsharp_web.js', baseUrl).href;
    const wasmUrl = new URL('/wasm/goonsharp_web_bg.wasm', baseUrl).href;
    const mod = await import(/* @vite-ignore */ jsUrl);
    await mod.default(wasmUrl);
    wasmModule = mod;
  } catch (e) {
    wasmError = `Failed to load WASM module: ${e}`;
    console.error('WASM load error:', e);
  } finally {
    wasmLoading = false;
  }
}

function compile(source: string): { success: boolean; rust_code: string; errors: string } {
  if (!wasmModule) {
    return { success: false, rust_code: '', errors: 'WASM module not loaded' };
  }
  try {
    const result = wasmModule.compile_goonsharp(source);
    const output = {
      success: result.success as boolean,
      rust_code: result.rust_code as string,
      errors: result.errors as string,
    };
    result.free();
    return output;
  } catch (e) {
    return { success: false, rust_code: '', errors: `Compiler error: ${e}` };
  }
}

export function renderPlayground(): string {
  const exampleOptions = Object.entries(EXAMPLES)
    .map(([key, ex]) => `<option value="${key}">${ex.name}</option>`)
    .join('');

  return `
    <div class="playground">
      <div class="playground-toolbar">
        <select id="example-select" title="Load example">
          ${exampleOptions}
        </select>
        <button class="run-btn" id="compile-btn" title="Compile (Ctrl+Enter)">
          ▶ Compile
        </button>
        <span id="wasm-status" style="font-size: 0.8rem; color: var(--text-muted);">
          Loading WASM...
        </span>
        <button class="share-btn" id="share-btn" title="Copy shareable URL">
          🔗 Share
        </button>
      </div>
      <div class="playground-panels">
        <div class="playground-panel">
          <div class="panel-header">
            <span class="dot input"></span>
            GoonSharp Source
          </div>
          <textarea
            class="editor"
            id="source-editor"
            spellcheck="false"
            autocomplete="off"
            autocorrect="off"
            autocapitalize="off"
            placeholder="Write your GoonSharp code here..."
          ></textarea>
        </div>
        <div class="playground-panel">
          <div class="panel-header">
            <span class="dot output" id="output-dot"></span>
            <span id="output-label">Output</span>
            <div class="panel-tabs">
              <button class="panel-tab active" data-tab="rust" id="tab-rust">Rust</button>
              <button class="panel-tab" data-tab="errors" id="tab-errors">Errors</button>
            </div>
          </div>
          <div class="output-area" id="output-area">
            <span style="color: var(--text-muted);">Click "Compile" or press Ctrl+Enter to transpile your GoonSharp code to Rust.</span>
          </div>
        </div>
      </div>
    </div>
  `;
}

let currentTab: 'rust' | 'errors' = 'rust';
let lastResult: { success: boolean; rust_code: string; errors: string } | null = null;

export function initPlayground(): void {
  const editor = document.getElementById('source-editor') as HTMLTextAreaElement;
  const compileBtn = document.getElementById('compile-btn')!;
  const exampleSelect = document.getElementById('example-select') as HTMLSelectElement;
  const shareBtn = document.getElementById('share-btn')!;
  const statusEl = document.getElementById('wasm-status')!;

  if (!editor) return;

  // Load initial example or from URL hash
  const params = new URLSearchParams(window.location.hash.split('?')[1] || '');
  const sharedCode = params.get('code');
  if (sharedCode) {
    editor.value = decodeURIComponent(atob(sharedCode));
  } else {
    editor.value = EXAMPLES.hello.code;
  }

  // Load WASM
  loadWasm().then(() => {
    if (wasmError) {
      statusEl.textContent = '❌ WASM failed';
      statusEl.style.color = 'var(--red)';
      statusEl.title = wasmError;
    } else {
      const version = wasmModule.get_version?.() || '69.0.0';
      statusEl.textContent = `✓ v${version}`;
      statusEl.style.color = 'var(--green)';
    }
  });

  // Compile button
  compileBtn.addEventListener('click', () => doCompile(editor));

  // Keyboard shortcut
  editor.addEventListener('keydown', (e) => {
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
      e.preventDefault();
      doCompile(editor);
    }
    // Tab key inserts spaces
    if (e.key === 'Tab') {
      e.preventDefault();
      const start = editor.selectionStart;
      const end = editor.selectionEnd;
      editor.value = editor.value.substring(0, start) + '    ' + editor.value.substring(end);
      editor.selectionStart = editor.selectionEnd = start + 4;
    }
  });

  // Example selector
  exampleSelect.addEventListener('change', () => {
    const example = EXAMPLES[exampleSelect.value];
    if (example) {
      editor.value = example.code;
      lastResult = null;
      showPlaceholder();
    }
  });

  // Share button
  shareBtn.addEventListener('click', () => {
    const code = btoa(encodeURIComponent(editor.value));
    const url = `${window.location.origin}${window.location.pathname}#/playground?code=${code}`;
    navigator.clipboard.writeText(url).then(() => {
      shareBtn.textContent = '✓ Copied!';
      setTimeout(() => { shareBtn.textContent = '🔗 Share'; }, 2000);
    });
  });

  // Tab switching
  document.getElementById('tab-rust')?.addEventListener('click', () => switchTab('rust'));
  document.getElementById('tab-errors')?.addEventListener('click', () => switchTab('errors'));
}

function doCompile(editor: HTMLTextAreaElement): void {
  const source = editor.value;
  const result = compile(source);
  lastResult = result;

  const dot = document.getElementById('output-dot')!;
  if (result.success) {
    dot.className = 'dot output';
    currentTab = 'rust';
  } else {
    dot.className = 'dot error';
    currentTab = 'errors';
  }

  updateTabs();
  renderOutput();
}

function switchTab(tab: 'rust' | 'errors'): void {
  currentTab = tab;
  updateTabs();
  renderOutput();
}

function updateTabs(): void {
  document.querySelectorAll('.panel-tab').forEach(t => t.classList.remove('active'));
  document.getElementById(`tab-${currentTab}`)?.classList.add('active');
}

function renderOutput(): void {
  const area = document.getElementById('output-area')!;

  if (!lastResult) {
    showPlaceholder();
    return;
  }

  if (currentTab === 'rust') {
    if (lastResult.success && lastResult.rust_code) {
      area.className = 'output-area success';
      area.innerHTML = `<div class="rust-output">${escapeHtml(lastResult.rust_code)}</div>`;
    } else if (!lastResult.success) {
      area.className = 'output-area';
      area.innerHTML = `<span style="color: var(--text-muted);">Compilation failed — check the Errors tab.</span>`;
    } else {
      area.className = 'output-area';
      area.innerHTML = `<span style="color: var(--text-muted);">No Rust output generated.</span>`;
    }
  } else {
    if (lastResult.errors) {
      area.className = 'output-area error';
      area.textContent = lastResult.errors;
    } else {
      area.className = 'output-area';
      area.innerHTML = `<span style="color: var(--green);">✓ No errors</span>`;
    }
  }
}

function showPlaceholder(): void {
  const area = document.getElementById('output-area');
  if (area) {
    area.className = 'output-area';
    area.innerHTML = `<span style="color: var(--text-muted);">Click "Compile" or press Ctrl+Enter to transpile your GoonSharp code to Rust.</span>`;
  }
}

function escapeHtml(str: string): string {
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

export function destroyPlayground(): void {
  // Cleanup if needed
  lastResult = null;
  currentTab = 'rust';
}
