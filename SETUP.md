# GoonSharp — Setup & Build Checklist

Current status: the VS Code extension is packaged and installable. The Rust crates have **not** been fully built or tested yet.

---

## Prerequisites

- **Rust toolchain** — `rustup` with stable channel (`rustc`, `cargo`)
- **wasm-pack** — for the web playground (`cargo install wasm-pack`)
- **Node.js / Bun** — for the VS Code extension packaging (`npm i -g @vscode/vsce`)
- **mdBook** — for building docs (`cargo install mdbook`)

---

## Crate Dependency Order

Build bottom-up — each layer depends on the one above it:

```
1. goonsharp-parser   (lexer + parser + AST)
2. goonsharp-codegen  (AST → Rust transpiler)
3. goonsharp (CLI)    (compiler binary)
   goonhub            (package manager binary)
   goonsharp-web      (WASM playground)
4. goonui             (standalone — egui UI framework)
```

---

## Step-by-Step Build

### 1. Build the parser (core library)

```bash
cargo build -p goonsharp-parser
```

**Known issue:** chumsky 0.9.x has extremely slow compile times due to deep monomorphization. First build may take 5–10 minutes. This is normal.

**Status:** ⬜ Had type errors (ParserInput → Token fix applied, comment parser fix applied). Needs a clean build to confirm.

### 2. Build the codegen (transpiler)

```bash
cargo build -p goonsharp-codegen
```

**Status:** ⬜ Not yet built. Depends on parser compiling first.

### 3. Build the CLI

```bash
cargo build -p goonsharp
```

Test it:
```bash
cargo run -p goonsharp -- examples/hello.goons
cargo run -p goonsharp -- examples/fibonacci.goons
cargo run -p goonsharp -- examples/error_handling.goons
cargo run -p goonsharp -- examples/traits_demo.goons
cargo run -p goonsharp -- examples/closures_iterators.goons
```

**Status:** ⬜ Not yet built.

### 4. Build GoonHub (package manager)

```bash
cargo build -p goonhub
```

Test it:
```bash
cargo run -p goonhub -- new test_project
cd test_project
cargo run -p goonhub -- run
cd .. && rm -rf test_project
```

**Status:** ⬜ Not yet built.

### 5. Build GoonUI (UI framework)

```bash
cargo build -p goonui
```

**Status:** ⬜ Not yet built. Standalone crate — depends on `eframe`/`egui` 0.29, not on the parser.

### 6. Build the Web Playground (WASM)

```bash
wasm-pack build crates/goonsharp-web --target web --out-dir ../../playground/pkg
```

Then serve `crates/goonsharp-web/index.html` locally to test.

**Status:** ⬜ Not yet built.

---

## Full Workspace Build (all at once)

```bash
cargo build --workspace
```

This builds everything except the WASM target (that needs `wasm-pack`).

For release:
```bash
cargo build --workspace --release
```

---

## Testing

### Run all tests (once they exist)
```bash
cargo test --workspace
```

### Manual smoke tests
- [ ] `goonsharp hello.goons` produces correct Rust output
- [ ] `goonsharp --emit-rust hello.goons` shows transpiled code
- [ ] `goonhub new myproject` scaffolds a project with `Goon.toml` + `src/main.goons`
- [ ] `goonhub run` inside a project compiles and runs
- [ ] Web playground compiles `.goons` to AST/Rust in browser

---

## VS Code Extension

Already packaged at `editors/vscode/goonsharp-69.0.0.vsix`.

Install:
```bash
code --install-extension editors/vscode/goonsharp-69.0.0.vsix
```

Repackage after changes:
```bash
cd editors/vscode
vsce package
```

**Status:** ✅ Installed and working (syntax highlighting, themes, icons, snippets).

---

## Docs

```bash
cd docs
mdbook build    # builds to docs/book/
mdbook serve    # live preview at localhost:3000
```

**Status:** ⬜ Not yet built/verified.

---

## Summary

| Component | Crate | Status |
|---|---|---|
| Parser | `goonsharp-parser` | ⬜ Needs clean build |
| Codegen | `goonsharp-codegen` | ⬜ Not built |
| CLI | `goonsharp` | ⬜ Not built |
| GoonHub | `goonhub` | ⬜ Not built |
| GoonUI | `goonui` | ⬜ Not built |
| Web Playground | `goonsharp-web` | ⬜ Not built |
| VS Code Extension | — | ✅ Done |
| Docs (mdBook) | — | ⬜ Not verified |
