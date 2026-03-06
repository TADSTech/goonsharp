# GoonSharp — Setup & Build Checklist

All 6 workspace crates compile successfully. The VS Code extension is packaged and installed. The web playground requires `wasm-pack`.

---

## Prerequisites

- **Rust toolchain** — `rustup` with stable channel (`rustc`, `cargo`)
- **wasm-pack** — for the web playground (`cargo install wasm-pack` — not in Fedora repos)
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

Compiles in ~2s thanks to aggressive `.boxed()` on every chumsky combinator chain. The CLI and GoonHub binaries spawn a 64MB-stack thread to handle chumsky's deep boxed-vtable recursion at runtime.

**Status:** ✅ Builds clean.

### 2. Build the codegen (transpiler)

```bash
cargo build -p goonsharp-codegen
```

**Status:** ✅ Builds clean.

### 3. Build the CLI

```bash
cargo build -p goonsharp
```

Test it with examples:
```bash
cargo run -p goonsharp -- examples/hello_goon.goons
cargo run -p goonsharp -- examples/fizzbuzz.goons
cargo run -p goonsharp -- examples/coom_counter.goons
cargo run -p goonsharp -- examples/enum_match.goons
cargo run -p goonsharp -- examples/error_handling.goons
cargo run -p goonsharp -- examples/goonstruct_demo.goons
cargo run -p goonsharp -- examples/traits_demo.goons
cargo run -p goonsharp -- examples/generics.goons
cargo run -p goonsharp -- examples/closures_iterators.goons
cargo run -p goonsharp -- examples/goon_game.goons
```

Show transpiled Rust:
```bash
cargo run -p goonsharp -- emit-rust examples/hello_goon.goons
```

**Status:** ✅ Builds clean. Runtime parsing needs testing per example.

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

**Status:** ✅ Builds clean. `goonhub new` scaffolds projects correctly.

### 5. Build GoonUI (UI framework)

```bash
cargo build -p goonui
```

**Status:** ✅ Builds clean. Standalone crate — depends on `eframe`/`egui` 0.29, not on the parser.

### 6. Build the Web Playground (WASM)

```bash
cargo install wasm-pack   # one-time setup (not in Fedora repos)
wasm-pack build crates/goonsharp-web --target web --out-dir ../../playground/pkg
```

Then serve `crates/goonsharp-web/index.html` locally to test.

**Status:** ⬜ Needs `wasm-pack` installed (`cargo install wasm-pack`).

---

## Full Workspace Build (all at once)

```bash
cargo build --workspace
```

This builds everything except the WASM target (that needs `wasm-pack`). Currently completes in under 1 second for incremental builds.

For release:
```bash
cargo build --workspace --release
```

---

## Available Examples

| File | Description |
|---|---|
| `examples/hello_goon.goons` | Hello World — simplest program |
| `examples/fizzbuzz.goons` | FizzBuzz with goon keywords |
| `examples/coom_counter.goons` | Loop/break (coom) demo |
| `examples/enum_match.goons` | Enum + goonmatch |
| `examples/error_handling.goons` | Result/Option patterns |
| `examples/goonstruct_demo.goons` | Struct definitions |
| `examples/traits_demo.goons` | Trait impl |
| `examples/generics.goons` | Generic types |
| `examples/closures_iterators.goons` | Closures + iterators |
| `examples/goon_game.goons` | Larger example |

---

## Testing

### Run all tests
```bash
cargo test --workspace
```

### Manual smoke tests
- [ ] `goonsharp examples/hello_goon.goons` — parses, transpiles, compiles, and runs
- [ ] `goonsharp emit-rust examples/hello_goon.goons` — shows transpiled Rust code
- [ ] `goonhub new myproject` — scaffolds project with `Goon.toml` + `src/main.goons`
- [ ] `goonhub run` inside a project — compiles and runs the `.goons` entry point
- [ ] Web playground compiles `.goons` to AST/Rust in browser (needs wasm-pack)

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

**Status:** ✅ Installed and working (syntax highlighting, DarkGoon + GoonLight themes, file icons, snippets, easter egg highlights).

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
| Parser | `goonsharp-parser` | ✅ Builds clean |
| Codegen | `goonsharp-codegen` | ✅ Builds clean |
| CLI | `goonsharp` | ✅ Builds clean |
| GoonHub | `goonhub` | ✅ Builds clean |
| GoonUI | `goonui` | ✅ Builds clean |
| Web Playground | `goonsharp-web` | ⬜ Needs `wasm-pack` |
| VS Code Extension | — | ✅ Done |
| Docs (mdBook) | — | ⬜ Not verified |

---

## Key Fixes Applied

- **Compile time**: Added `.boxed()` to every chumsky combinator chain — reduced parser compile from hours to ~2s
- **Stack overflow**: CLI and GoonHub spawn a 64MB-stack thread for chumsky's deep boxed-vtable recursion
- **Infinite recursion**: Broke `type_parser() ↔ expr_parser_inner()` mutual recursion cycle by using a simplified expression parser for array sizes in type annotations
- **Box destructuring**: Fixed codegen pattern matching through `Box<Spanned<T>>` types
- **API cleanup**: Removed unused imports, fixed float literals, fixed `Option` move-after-use
