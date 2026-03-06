# GoonSharp 🟣

> The ultimate shitpost programming language — transpiles to Rust

## Install

```bash
npm install -g goonsharp
```

**Requires:** Linux x64 (macOS/Windows coming soon™)

## Quick Start

```bash
# Create a new project
goonhub new my_goon_project
cd my_goon_project

# Or just run a file
echo 'goonsesh main() { goonprint!("Hello, Goon World! 🟣"); }' > hello.goons
goonsharp hello.goons
```

## Commands

### `goonsharp` — Compiler

| Command | Description |
|---------|-------------|
| `goonsharp <file.goons>` | Compile and run |
| `goonsharp build <file.goons>` | Compile only |
| `goonsharp check <file.goons>` | Parse check only |
| `goonsharp emit-rust <file.goons>` | Show transpiled Rust |
| `goonsharp fmt <file.goons>` | Format (lol) |

### `goonhub` — Package Manager

| Command | Description |
|---------|-------------|
| `goonhub new <name>` | Create a new project |
| `goonhub build` | Build the project |
| `goonhub run` | Build and run |
| `goonhub test` | Run tests |
| `goonhub add <dep>` | Add a dependency |

## Language Cheat Sheet

```goonsharp
// Functions
goonsesh main() {
    goonprint!("Hello Goon!");
}

// Variables
goonlet x = 69;
goonlet mut y = 420;

// Control flow
goonif (x > 10) {
    goonprint!("nice");
} goonnah {
    goonprint!("not nice");
}

// Loops
goonfor i goonin 1..=10 {
    goonprint!("{}", i);
}

goonloop {
    goonyeet; // break
}

// Structs
goonstruct Player {
    name: GoonString,
    score: i69,  // it's an alias for i64
}
```

## Links

- **Website:** [goonsharp.dev](https://goonsharp.dev)
- **Playground:** [goonsharp.dev/#/playground](https://goonsharp.dev/#/playground)
- **VS Code Extension:** Search "GoonSharp" in the marketplace
- **Source:** [github.com/goonsharp/goonsharp](https://github.com/goonsharp/goonsharp)

## License

MIT OR Unlicense — because freedom is goon.
