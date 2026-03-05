# Installation

## Prerequisites

GoonSharp transpiles to Rust, so you need the Rust toolchain installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install GoonSharp

### From source (recommended for now)

```bash
git clone https://github.com/goonsharp/goonsharp.git
cd goonsharp
cargo install --path crates/goonsharp-cli
cargo install --path crates/goonhub
```

This gives you two commands:
- `goonsharp` — the compiler
- `goonhub` — the package manager

### Verify installation

```bash
goonsharp --version
# GoonSharp v69.0.0

goonhub --version
# GoonHub v69.0.0
```

## VS Code Extension

Install the GoonSharp extension for syntax highlighting and the DarkGoon theme:

1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Search for "GoonSharp"
4. Click Install
5. Set color theme to "DarkGoon" for maximum vibes

## Editor Support

- **VS Code**: Full support via official extension
- **Neovim**: Tree-sitter grammar available (see `editors/` folder)
- **Other editors**: TextMate grammar compatible (`.tmLanguage.json`)
