export function renderDocs(): string {
  return `
    <div class="docs-page">
      <h1>Documentation</h1>
      <p class="lead">Everything you need to become a certified GoonSharp developer.</p>

      <!-- Table of Contents -->
      <div class="docs-toc">
        <h2>📑 Contents</h2>
        <ul>
          <li><a href="#basics">Basics</a></li>
          <li><a href="#variables">Variables</a></li>
          <li><a href="#types">Types</a></li>
          <li><a href="#functions">Functions</a></li>
          <li><a href="#control-flow">Control Flow</a></li>
          <li><a href="#loops">Loops</a></li>
          <li><a href="#structs">Structs</a></li>
          <li><a href="#enums">Enums & Match</a></li>
          <li><a href="#error-handling">Error Handling</a></li>
          <li><a href="#closures">Closures & Iterators</a></li>
          <li><a href="#traits">Traits</a></li>
          <li><a href="#keyword-map">Keyword Map</a></li>
        </ul>
      </div>

      <!-- Basics -->
      <div class="docs-section" id="basics">
        <h2>Basics</h2>
        <p>GoonSharp is a transpiled language — your <code>.goons</code> source compiles to Rust, which then compiles to native machine code. The GoonSharp compiler handles all of this for you.</p>
        <p>Every GoonSharp program starts with a <code>goonsesh main()</code> function:</p>
        <div class="code-block"><span class="cmt">// This is a comment</span>
<span class="kw">goonsesh</span> <span class="fn">main</span>() {
    <span class="mac">goonprint!</span>(<span class="str">"Hello, Goon World!"</span>);
}</div>
        <p>Files use the <code>.goons</code> extension. Run them with <code>goonsharp myfile.goons</code> or build a project with <code>goonhub run</code>.</p>
      </div>

      <!-- Variables -->
      <div class="docs-section" id="variables">
        <h2>Variables</h2>
        <p>Declare variables with <code>goonlet</code>. Variables are immutable by default — use <code>goonlet mut</code> for mutability.</p>
        <div class="code-block"><span class="kw">goonlet</span> x = <span class="num">42</span>;          <span class="cmt">// immutable</span>
<span class="kw">goonlet mut</span> y = <span class="num">0</span>;       <span class="cmt">// mutable</span>
y = y + <span class="num">1</span>;

<span class="kw">goonlet</span> name: <span class="ty">GoonString</span> = <span class="str">"Alice"</span>.to_string();
<span class="kw">goonlet</span> pi: <span class="ty">f64</span> = <span class="num">3.14159</span>;</div>
      </div>

      <!-- Types -->
      <div class="docs-section" id="types">
        <h2>Types</h2>
        <p>GoonSharp supports all Rust primitive types plus some goon aliases:</p>
        <table class="docs-table">
          <thead>
            <tr><th>GoonSharp Type</th><th>Rust Equivalent</th><th>Description</th></tr>
          </thead>
          <tbody>
            <tr><td><code>i69</code></td><td><code>i64</code></td><td>The One True Integer™</td></tr>
            <tr><td><code>GoonString</code></td><td><code>String</code></td><td>Owned string</td></tr>
            <tr><td><code>goobool</code></td><td><code>bool</code></td><td>Boolean</td></tr>
            <tr><td><code>i32</code>, <code>u64</code>, etc.</td><td>same</td><td>Standard Rust integers</td></tr>
            <tr><td><code>f32</code>, <code>f64</code></td><td>same</td><td>Floating point</td></tr>
            <tr><td><code>&str</code></td><td>same</td><td>String slice</td></tr>
            <tr><td><code>Vec&lt;T&gt;</code></td><td>same</td><td>Dynamic array</td></tr>
            <tr><td><code>Option&lt;T&gt;</code></td><td>same</td><td>Optional value</td></tr>
            <tr><td><code>Result&lt;T,E&gt;</code></td><td>same</td><td>Error handling</td></tr>
          </tbody>
        </table>
      </div>

      <!-- Functions -->
      <div class="docs-section" id="functions">
        <h2>Functions</h2>
        <p>Declare functions with <code>goonsesh</code>:</p>
        <div class="code-block"><span class="kw">goonsesh</span> <span class="fn">add</span>(a: <span class="ty">i69</span>, b: <span class="ty">i69</span>) -> <span class="ty">i69</span> {
    a + b
}

<span class="kw">goonsesh</span> <span class="fn">greet</span>(name: &<span class="ty">str</span>) {
    <span class="mac">goonprint!</span>(<span class="str">"What's up, {}!"</span>, name);
}

<span class="cmt">// Public functions</span>
<span class="kw">goonsquad goonsesh</span> <span class="fn">public_fn</span>() {
    <span class="cmt">// goonsquad = pub</span>
}</div>
      </div>

      <!-- Control flow -->
      <div class="docs-section" id="control-flow">
        <h2>Control Flow</h2>
        <h3>If / Else</h3>
        <div class="code-block"><span class="kw">goonif</span> (score > <span class="num">100</span>) {
    <span class="mac">goonprint!</span>(<span class="str">"HIGH SCORE"</span>);
} <span class="kw">goonnah goonif</span> (score > <span class="num">50</span>) {
    <span class="mac">goonprint!</span>(<span class="str">"mid"</span>);
} <span class="kw">goonnah</span> {
    <span class="mac">goonprint!</span>(<span class="str">"L"</span>);
}</div>
        <h3>Match</h3>
        <div class="code-block"><span class="kw">goonmatch</span> value {
    <span class="num">1</span> => <span class="mac">goonprint!</span>(<span class="str">"one"</span>),
    <span class="num">2</span> | <span class="num">3</span> => <span class="mac">goonprint!</span>(<span class="str">"two or three"</span>),
    _ => <span class="mac">goonprint!</span>(<span class="str">"something else"</span>),
}</div>
      </div>

      <!-- Loops -->
      <div class="docs-section" id="loops">
        <h2>Loops</h2>
        <h3>For Loop</h3>
        <div class="code-block"><span class="kw">goonfor</span> i <span class="kw">goonin</span> <span class="num">0</span>..<span class="num">10</span> {
    <span class="mac">goonprint!</span>(<span class="str">"{}"</span>, i);
}

<span class="kw">goonfor</span> item <span class="kw">goonin</span> vec![<span class="num">1</span>, <span class="num">2</span>, <span class="num">3</span>] {
    <span class="mac">goonprint!</span>(<span class="str">"{}"</span>, item);
}</div>
        <h3>While Loop</h3>
        <div class="code-block"><span class="kw">goonlet mut</span> n = <span class="num">0</span>;
<span class="kw">goonwhile</span> (n < <span class="num">5</span>) {
    n = n + <span class="num">1</span>;
}</div>
        <h3>Infinite Loop</h3>
        <div class="code-block"><span class="kw">goonloop</span> {
    <span class="mac">goonprint!</span>(<span class="str">"forever..."</span>);
    <span class="kw">goonyeet</span>;  <span class="cmt">// break</span>
}

<span class="cmt">// gooncontinue = continue, goonreturn = return</span></div>
      </div>

      <!-- Structs -->
      <div class="docs-section" id="structs">
        <h2>Structs</h2>
        <div class="code-block"><span class="kw">goonstruct</span> <span class="ty">Player</span> {
    name: <span class="ty">GoonString</span>,
    score: <span class="ty">i69</span>,
    alive: <span class="ty">goobool</span>,
}

<span class="kw">goonimpl</span> <span class="ty">Player</span> {
    <span class="kw">goonsesh</span> <span class="fn">new</span>(name: <span class="ty">GoonString</span>) -> <span class="ty">Player</span> {
        <span class="ty">Player</span> { name, score: <span class="num">0</span>, alive: <span class="kw">fax</span> }
    }

    <span class="kw">goonsesh</span> <span class="fn">is_goated</span>(&self) -> <span class="ty">goobool</span> {
        self.score > <span class="num">1000</span>
    }
}</div>
        <p>Note: <code>fax</code> = <code>true</code>, <code>cap</code> = <code>false</code> in GoonSharp.</p>
      </div>

      <!-- Enums -->
      <div class="docs-section" id="enums">
        <h2>Enums & Match</h2>
        <div class="code-block"><span class="kw">goonenum</span> <span class="ty">Weapon</span> {
    Sword,
    Bow(<span class="ty">i69</span>),   <span class="cmt">// arrow count</span>
    Magic { mana: <span class="ty">i69</span>, element: <span class="ty">GoonString</span> },
}

<span class="kw">goonsesh</span> <span class="fn">describe</span>(w: <span class="ty">Weapon</span>) {
    <span class="kw">goonmatch</span> w {
        <span class="ty">Weapon</span>::Sword => <span class="mac">goonprint!</span>(<span class="str">"🗡️ Sword"</span>),
        <span class="ty">Weapon</span>::Bow(n) => <span class="mac">goonprint!</span>(<span class="str">"🏹 Bow with {} arrows"</span>, n),
        <span class="ty">Weapon</span>::Magic { mana, element } => {
            <span class="mac">goonprint!</span>(<span class="str">"✨ {} magic ({} mana)"</span>, element, mana);
        }
    }
}</div>
      </div>

      <!-- Error Handling -->
      <div class="docs-section" id="error-handling">
        <h2>Error Handling</h2>
        <div class="code-block"><span class="kw">goonsesh</span> <span class="fn">risky</span>() -> Result&lt;<span class="ty">i69</span>, <span class="ty">GoonString</span>&gt; {
    <span class="kw">goonlet</span> val = might_fail()?;  <span class="cmt">// ? propagates errors</span>
    Ok(val * <span class="num">2</span>)
}

<span class="kw">goonsesh</span> <span class="fn">main</span>() {
    <span class="kw">goonmatch</span> risky() {
        Ok(v) => <span class="mac">goonprint!</span>(<span class="str">"Got: {}"</span>, v),
        Err(e) => <span class="mac">goonprint!</span>(<span class="str">"Error: {}"</span>, e),
    }
}</div>
      </div>

      <!-- Closures -->
      <div class="docs-section" id="closures">
        <h2>Closures & Iterators</h2>
        <div class="code-block"><span class="kw">goonlet</span> nums = vec![<span class="num">1</span>, <span class="num">2</span>, <span class="num">3</span>, <span class="num">4</span>, <span class="num">5</span>];

<span class="cmt">// Closures</span>
<span class="kw">goonlet</span> doubled: Vec&lt;<span class="ty">i69</span>&gt; = nums.iter()
    .map(|x| x * <span class="num">2</span>)
    .collect();

<span class="cmt">// Filter + collect</span>
<span class="kw">goonlet</span> evens: Vec&lt;&<span class="ty">i69</span>&gt; = nums.iter()
    .filter(|x| *x % <span class="num">2</span> == <span class="num">0</span>)
    .collect();

<span class="cmt">// Named closure</span>
<span class="kw">goonlet</span> add = |a: <span class="ty">i69</span>, b: <span class="ty">i69</span>| -> <span class="ty">i69</span> { a + b };
<span class="mac">goonprint!</span>(<span class="str">"{}"</span>, add(<span class="num">2</span>, <span class="num">3</span>));</div>
      </div>

      <!-- Traits -->
      <div class="docs-section" id="traits">
        <h2>Traits</h2>
        <div class="code-block"><span class="kw">goontrait</span> <span class="ty">Describable</span> {
    <span class="kw">goonsesh</span> <span class="fn">describe</span>(&self) -> <span class="ty">GoonString</span>;
    <span class="kw">goonsesh</span> <span class="fn">vibe_check</span>(&self) -> <span class="ty">goobool</span> {
        <span class="kw">fax</span>  <span class="cmt">// default impl</span>
    }
}

<span class="kw">goonimpl</span> <span class="ty">Describable</span> <span class="kw">goonfor</span> <span class="ty">Player</span> {
    <span class="kw">goonsesh</span> <span class="fn">describe</span>(&self) -> <span class="ty">GoonString</span> {
        format!(<span class="str">"Player '{}' (score: {})"</span>, self.name, self.score)
    }
}</div>
      </div>

      <!-- Keyword Map -->
      <div class="docs-section" id="keyword-map">
        <h2>Keyword Map</h2>
        <p>Complete mapping of GoonSharp keywords to their Rust equivalents:</p>
        <table class="docs-table">
          <thead>
            <tr><th>GoonSharp</th><th>Rust</th><th>What it do</th></tr>
          </thead>
          <tbody>
            <tr><td><code>goonsesh</code></td><td><code>fn</code></td><td>Function declaration</td></tr>
            <tr><td><code>goonlet</code></td><td><code>let</code></td><td>Variable binding</td></tr>
            <tr><td><code>goonif</code></td><td><code>if</code></td><td>Conditional</td></tr>
            <tr><td><code>goonnah</code></td><td><code>else</code></td><td>Else branch</td></tr>
            <tr><td><code>goonfor</code></td><td><code>for</code></td><td>For loop</td></tr>
            <tr><td><code>goonin</code></td><td><code>in</code></td><td>Iterator binding</td></tr>
            <tr><td><code>goonwhile</code></td><td><code>while</code></td><td>While loop</td></tr>
            <tr><td><code>goonloop</code></td><td><code>loop</code></td><td>Infinite loop</td></tr>
            <tr><td><code>goonyeet</code></td><td><code>break</code></td><td>Break out of loop</td></tr>
            <tr><td><code>gooncontinue</code></td><td><code>continue</code></td><td>Skip iteration</td></tr>
            <tr><td><code>goonreturn</code></td><td><code>return</code></td><td>Return from function</td></tr>
            <tr><td><code>goonmatch</code></td><td><code>match</code></td><td>Pattern matching</td></tr>
            <tr><td><code>goonstruct</code></td><td><code>struct</code></td><td>Struct definition</td></tr>
            <tr><td><code>goonenum</code></td><td><code>enum</code></td><td>Enum definition</td></tr>
            <tr><td><code>goonimpl</code></td><td><code>impl</code></td><td>Implementation block</td></tr>
            <tr><td><code>goontrait</code></td><td><code>trait</code></td><td>Trait definition</td></tr>
            <tr><td><code>goonsquad</code></td><td><code>pub</code></td><td>Public visibility</td></tr>
            <tr><td><code>goonuse</code></td><td><code>use</code></td><td>Import/use</td></tr>
            <tr><td><code>goonmod</code></td><td><code>mod</code></td><td>Module declaration</td></tr>
            <tr><td><code>fax</code></td><td><code>true</code></td><td>Boolean true</td></tr>
            <tr><td><code>cap</code></td><td><code>false</code></td><td>Boolean false</td></tr>
            <tr><td><code>nocap</code></td><td><code>true</code></td><td>Also true (synonym)</td></tr>
            <tr><td><code>goonprint!</code></td><td><code>println!</code></td><td>Print with newline</td></tr>
            <tr><td><code>i69</code></td><td><code>i64</code></td><td>Nice integer type</td></tr>
            <tr><td><code>GoonString</code></td><td><code>String</code></td><td>Owned string</td></tr>
            <tr><td><code>goobool</code></td><td><code>bool</code></td><td>Boolean type</td></tr>
          </tbody>
        </table>
      </div>

      <!-- CLI Reference -->
      <div class="docs-section" id="cli">
        <h2>CLI Reference</h2>
        <h3>goonsharp (Compiler)</h3>
        <table class="docs-table">
          <thead>
            <tr><th>Command</th><th>Description</th></tr>
          </thead>
          <tbody>
            <tr><td><code>goonsharp &lt;file.goons&gt;</code></td><td>Compile and run a file</td></tr>
            <tr><td><code>goonsharp build &lt;file&gt;</code></td><td>Compile only (produces binary)</td></tr>
            <tr><td><code>goonsharp check &lt;file&gt;</code></td><td>Parse check (no compilation)</td></tr>
            <tr><td><code>goonsharp emit-rust &lt;file&gt;</code></td><td>Show generated Rust code</td></tr>
            <tr><td><code>goonsharp fmt &lt;file&gt;</code></td><td>Format source (cosmetic only)</td></tr>
          </tbody>
        </table>
        <h3>goonhub (Package Manager)</h3>
        <table class="docs-table">
          <thead>
            <tr><th>Command</th><th>Description</th></tr>
          </thead>
          <tbody>
            <tr><td><code>goonhub new &lt;name&gt;</code></td><td>Create a new project</td></tr>
            <tr><td><code>goonhub init</code></td><td>Initialize in current directory</td></tr>
            <tr><td><code>goonhub build</code></td><td>Build the project</td></tr>
            <tr><td><code>goonhub run</code></td><td>Build and run</td></tr>
            <tr><td><code>goonhub test</code></td><td>Run tests</td></tr>
            <tr><td><code>goonhub add &lt;dep&gt;</code></td><td>Add a dependency</td></tr>
            <tr><td><code>goonhub publish</code></td><td>Publish to GoonHub</td></tr>
          </tbody>
        </table>
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
          <li><a href="#/install">Install</a></li>
          <li><a href="#/playground">Playground</a></li>
        </ul>
      </div>
    </footer>
  `;
}
