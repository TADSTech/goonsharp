export function renderInstall(): string {
  return `
    <div class="install-page">
      <h1>Installation</h1>
      <p class="lead">Get GoonSharp running on your machine in under a minute.</p>

      <div class="install-step">
        <h2><span class="step-num">1</span> Prerequisites</h2>
        <p>GoonSharp transpiles to Rust, so you'll need both <strong>Node.js</strong> (for the installer) and <strong>Rust</strong> (for compilation).</p>
        <div class="code-block" data-copy="curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh">curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</div>
        <p class="text-muted" style="font-size: 0.85rem; margin-top: 0.5rem;">
          Or on Fedora/RHEL: <code style="color: var(--cyan);">sudo dnf install rust cargo</code>
        </p>
      </div>

      <div class="install-step">
        <h2><span class="step-num">2</span> Install GoonSharp</h2>
        <p>Install the compiler and package manager globally via npm:</p>
        <div class="code-block" data-copy="npm install -g goonsharp">npm install -g goonsharp</div>
        <p style="margin-top: 0.75rem;">This installs two commands:</p>
        <ul style="color: var(--text-secondary); padding-left: 1.5rem; margin-top: 0.25rem;">
          <li><code style="color: var(--cyan);">goonsharp</code> — the compiler</li>
          <li><code style="color: var(--cyan);">goonhub</code> — the package manager</li>
        </ul>
      </div>

      <div class="install-step">
        <h2><span class="step-num">3</span> Create Your First Project</h2>
        <p>Scaffold a new GoonSharp project:</p>
        <div class="code-block" data-copy="goonhub new my_goon_project && cd my_goon_project">goonhub new my_goon_project
cd my_goon_project</div>
      </div>

      <div class="install-step">
        <h2><span class="step-num">4</span> Write Some Code</h2>
        <p>Open <code style="color: var(--cyan);">src/main.goons</code> and write:</p>
        <div class="code-block" style="color: var(--text-primary);">
<span class="cmt">// My first GoonSharp program</span>
<span class="kw">goonsesh</span> <span class="fn">main</span>() {
    <span class="mac">goonprint!</span>(<span class="str">"Hello, Goon World! 🟣"</span>);
}</div>
      </div>

      <div class="install-step">
        <h2><span class="step-num">5</span> Run It</h2>
        <p>Build and run your project:</p>
        <div class="code-block" data-copy="goonhub run">goonhub run</div>
        <p>Or compile a standalone file:</p>
        <div class="code-block" data-copy="goonsharp hello.goons">goonsharp hello.goons</div>
      </div>

      <div class="install-note">
        <strong>🐧 Linux Only (for now)</strong> — GoonSharp currently ships pre-built binaries for
        Linux x86_64. macOS and Windows support is coming soon™. You can also build from source on any
        platform that supports Rust.
      </div>

      <div class="install-step" style="margin-top: 2rem;">
        <h2><span class="step-num">⚙</span> VS Code Extension</h2>
        <p>Get syntax highlighting, themes, snippets, and file icons:</p>
        <div class="code-block" data-copy="code --install-extension goonsharp-69.0.0.vsix">code --install-extension goonsharp-69.0.0.vsix</div>
        <p style="margin-top: 0.5rem;">
          Or search <strong>"GoonSharp"</strong> in the VS Code extension marketplace.
          Includes DarkGoon (cyberpunk void) and GoonLight (lavender) themes.
        </p>
      </div>

      <div class="install-step" style="margin-top: 1.5rem;">
        <h2><span class="step-num">🔧</span> Build From Source</h2>
        <p>Clone and build the entire workspace:</p>
        <div class="code-block" data-copy="git clone https://github.com/goonsharp/goonsharp && cd goonsharp && cargo build --workspace --release">git clone https://github.com/goonsharp/goonsharp
cd goonsharp
cargo build --workspace --release</div>
        <p style="margin-top: 0.5rem;">
          Binaries will be at <code style="color: var(--cyan);">target/release/goonsharp</code> and
          <code style="color: var(--cyan);">target/release/goonhub</code>.
        </p>
      </div>
    </div>

    <!-- Footer -->
    <footer class="footer">
      <div class="footer-inner">
        <div class="footer-brand">
          <img src="/logo.svg" alt="" />
          <span>GoonSharp v69.0.0</span>
        </div>
        <ul class="footer-links">
          <li><a href="https://github.com/goonsharp/goonsharp" target="_blank" rel="noopener">GitHub</a></li>
          <li><a href="#/docs">Docs</a></li>
          <li><a href="#/playground">Playground</a></li>
        </ul>
      </div>
    </footer>
  `;
}
