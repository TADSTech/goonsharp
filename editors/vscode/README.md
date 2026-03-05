# GoonSharp Official — VS Code Extension 🟣

The **official** VS Code extension for [GoonSharp](https://github.com/goonsharp/goonsharp), the meme-powered programming language that transpiles to Rust.

## Features

### Syntax Highlighting
Full TextMate grammar for `.goons` files with support for all GoonSharp keywords, operators, macros, lifetimes, and string interpolation.

### Two Color Themes
- **DarkGoon** — cyberpunk void with neon pink keywords, electric blue functions, radioactive green strings
- **GoonLight** — clean lavender light theme with purple accents

### File Icons
Custom `.goons` file icons with light and dark variants. Activate via `Ctrl+Shift+P` → "File Icon Theme" → **GoonSharp Icons**.

### Snippets
Quick scaffolds for common patterns:
- `goonsesh` → main function
- `goonif` → if/else block
- `goonfor` → for loop
- `goonstruct` → struct definition
- `goonimpl` → impl block
- `goonmatch` → match expression
- ...and more

### Easter Eggs 🥚
Certain numbers and words get special highlighting:
- `69` — hot pink
- `420` — neon green
- `1337` — hacker green
- `sigma`, `rizz`, `skibidi` — glowing cyan
- `bruh`, `sheesh`, `bussin` — chaotic lime
- `gg`, `ez`, `ratio` — rage red

## Quick Start

1. Install the extension
2. Open any `.goons` file
3. Select a theme: `Ctrl+Shift+P` → "Color Theme" → **DarkGoon** or **GoonLight**
4. (Optional) Enable file icons: `Ctrl+Shift+P` → "File Icon Theme" → **GoonSharp Icons**

## Example

```goons
goonsesh main() {
    goon greeting = "hello, goon world! 🟣";
    goonprint!("{}", greeting);

    goon x = 69;  // nice
    goonif x > 420 {
        goonprint!("blazing");
    } goonelse {
        goonprint!("not yet");
    }
}
```

## GoonSharp Language

| GoonSharp | Rust Equivalent |
|-----------|----------------|
| `goonsesh` | `fn` |
| `goon` | `let` |
| `goonmut` | `mut` |
| `goonif` / `goonelse` | `if` / `else` |
| `goonfor` | `for` |
| `goonmatch` | `match` |
| `goonstruct` | `struct` |
| `goonenum` | `enum` |
| `goonimpl` | `impl` |
| `goontrait` | `trait` |
| `goonprint!` | `println!` |
| `ruin!` | `panic!` |
| `gooning` / `no_goon` | `true` / `false` |
| `coom` | `break` |
| `edging` | `while` |

## Requirements

- VS Code 1.80.0+
- For running `.goons` files: [GoonSharp CLI](https://github.com/goonsharp/goonsharp)

## License

MIT — see [LICENSE.md](LICENSE.md)
