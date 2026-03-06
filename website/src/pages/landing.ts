export function renderLanding(): string {
  return `
    <!-- Hero -->
    <section class="hero">
      <div class="hero-content">
        <img class="hero-logo" src="/logo.svg" alt="GoonSharp Logo" />
        <h1 class="hero-title">GoonSharp</h1>
        <p class="hero-subtitle">
          The ultimate shitpost programming language — transpiles to Rust.
          Because every language starts as a meme, but only the real ones ship.
        </p>
        <div class="hero-actions">
          <a href="#/playground" class="btn btn-primary">▶ Try It Live</a>
          <a href="#/install" class="btn btn-secondary">📦 Install</a>
          <a href="#/docs" class="btn btn-secondary">📖 Docs</a>
        </div>
        <div class="install-command" title="Click to copy">
          <span class="prompt">$</span>
          <span class="cmd">npm install -g goonsharp</span>
          <span class="copy-icon">📋</span>
        </div>
      </div>
    </section>

    <!-- Code Showcase -->
    <section class="section">
      <div class="code-showcase">
        <div class="code-header">
          <span class="code-dot red"></span>
          <span class="code-dot yellow"></span>
          <span class="code-dot green"></span>
          <span>hello_goon.goons</span>
        </div>
        <div class="code-body">
<pre><span class="cmt">// Hello Goon — your first GoonSharp program</span>
<span class="kw">goonsesh</span> <span class="fn">main</span>() {
    <span class="kw">goonlet</span> name = <span class="str">"World"</span>;
    <span class="kw">goonlet</span> count = <span class="num">69</span>;

    <span class="kw">goonfor</span> i <span class="kw">goonin</span> <span class="num">0</span>..<span class="op">=</span>count {
        <span class="mac">goonprint!</span>(<span class="str">"Hello, {} #{}"</span>, name, i);
    }

    <span class="kw">goonif</span> (count <span class="op">==</span> <span class="num">69</span>) {
        <span class="mac">goonprint!</span>(<span class="str">"Nice. 🟣"</span>);
    }
}</pre>
        </div>
      </div>
    </section>

    <!-- Features -->
    <section class="section">
      <h2 class="section-title">Why GoonSharp?</h2>
      <p class="section-subtitle">All the vibes, none of the segfaults</p>
      <div class="features-grid">
        <div class="feature-card">
          <div class="feature-icon">⚡</div>
          <h3>Transpiles to Rust</h3>
          <p>Your meme code compiles into fast, safe Rust. Zero-cost abstractions meet zero-effort naming conventions.</p>
        </div>
        <div class="feature-card">
          <div class="feature-icon">🟣</div>
          <h3>Peak Naming</h3>
          <p><code>goonsesh</code> instead of <code>fn</code>. <code>goonfor</code> instead of <code>for</code>. <code>goonyeet</code> instead of <code>break</code>. You get the idea.</p>
        </div>
        <div class="feature-card">
          <div class="feature-icon">📦</div>
          <h3>GoonHub Package Manager</h3>
          <p>A built-in project manager and package tool. <code>goonhub new</code>, <code>goonhub build</code>, <code>goonhub run</code> — the whole shebang.</p>
        </div>
        <div class="feature-card">
          <div class="feature-icon">🎨</div>
          <h3>VS Code Extension</h3>
          <p>Full syntax highlighting, cyberpunk DarkGoon & GoonLight themes, snippets, and file icons. Install it from the marketplace.</p>
        </div>
        <div class="feature-card">
          <div class="feature-icon">🌐</div>
          <h3>Web Playground</h3>
          <p>Compile GoonSharp right in your browser with our WASM-powered playground. No install needed to vibe.</p>
        </div>
        <div class="feature-card">
          <div class="feature-icon">🔥</div>
          <h3>Version 69.0.0</h3>
          <p>We skipped straight to the good version number. Semantic versioning is a social construct.</p>
        </div>
      </div>
    </section>

    <!-- Quick Example Grid -->
    <section class="section">
      <h2 class="section-title">GoonSharp vs Rust</h2>
      <p class="section-subtitle">Same power, better vibes</p>
      <div class="features-grid" style="grid-template-columns: 1fr 1fr;">
        <div class="code-showcase">
          <div class="code-header">
            <span class="code-dot red"></span>
            <span class="code-dot yellow"></span>
            <span class="code-dot green"></span>
            <span>GoonSharp 🟣</span>
          </div>
          <div class="code-body">
<pre><span class="kw">goonstruct</span> <span class="ty">Player</span> {
    name: <span class="ty">GoonString</span>,
    score: <span class="ty">i69</span>,
}

<span class="kw">goonsesh</span> <span class="fn">main</span>() {
    <span class="kw">goonlet</span> p = <span class="ty">Player</span> {
        name: <span class="str">"xX_G00N_Xx"</span>.to_string(),
        score: <span class="num">420</span>,
    };
    <span class="mac">goonprint!</span>(<span class="str">"{}: {}"</span>, p.name, p.score);
}</pre>
          </div>
        </div>
        <div class="code-showcase">
          <div class="code-header">
            <span class="code-dot red"></span>
            <span class="code-dot yellow"></span>
            <span class="code-dot green"></span>
            <span>Transpiled Rust 🦀</span>
          </div>
          <div class="code-body">
<pre><span class="kw">struct</span> <span class="ty">Player</span> {
    name: <span class="ty">String</span>,
    score: <span class="ty">i64</span>,
}

<span class="kw">fn</span> <span class="fn">main</span>() {
    <span class="kw">let</span> p = <span class="ty">Player</span> {
        name: <span class="str">"xX_G00N_Xx"</span>.to_string(),
        score: <span class="num">420</span>,
    };
    <span class="mac">println!</span>(<span class="str">"{}: {}"</span>, p.name, p.score);
}</pre>
          </div>
        </div>
      </div>
    </section>

    <!-- CTA -->
    <section class="section" style="text-align: center; padding-bottom: 5rem;">
      <h2 class="section-title glow">Ready to Goon?</h2>
      <p class="section-subtitle">Join the revolution. Install GoonSharp today.</p>
      <div class="hero-actions">
        <a href="#/install" class="btn btn-primary">📦 Get Started</a>
        <a href="#/playground" class="btn btn-secondary">▶ Try in Browser</a>
      </div>
    </section>

    <!-- Footer -->
    <footer class="footer">
      <div class="footer-inner">
        <div class="footer-brand">
          <img src="/logo.svg" alt="" />
          <span>GoonSharp v69.0.0 — the ultimate shitpost language</span>
        </div>
        <ul class="footer-links">
          <li><a href="https://github.com/goonsharp/goonsharp" target="_blank" rel="noopener">GitHub</a></li>
          <li><a href="https://www.npmjs.com/package/goonsharp" target="_blank" rel="noopener">npm</a></li>
          <li><a href="#/docs">Docs</a></li>
          <li><a href="#/playground">Playground</a></li>
        </ul>
      </div>
    </footer>
  `;
}
