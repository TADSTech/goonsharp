# The GoonSharp Book 🟣

Welcome to the official documentation for **GoonSharp** — the ultimate shitpost programming language that actually compiles.

GoonSharp is a superset of Rust (sort of) where every keyword has been replaced with goon-themed alternatives. It features:

- **A real parser** built with chumsky — not just `str::replace` anymore
- **Full Rust transpilation** — your .goons files become valid Rust
- **Beautiful error messages** with ariadne — goon-flavored, of course
- **GoonUI** — a desktop UI framework built on egui
- **GoonHub** — a package manager and build system
- **VS Code extension** — with the DarkGoon theme

## Philosophy

Every great language starts as a joke. GoonSharp is no exception, but unlike most joke languages, it actually works. You can build real applications, use real Rust types, and ship real binaries — all while maintaining maximum goon energy.

## Quick Example

```goons
goonsesh main() {
    goon message = "welcome to the goon zone";
    goonprint!("{}", message);

    goonfor i goonin 0..10 {
        goonif (i % 2 == 0) {
            goonprint!("{} is based", i);
        }
    }
}
```

This transpiles to valid Rust and compiles to a native binary. No interpreter needed.

## Getting Started

Ready to goon? Start with [Installation](./installation.md) and then build your [first goonsesh](./first-goonsesh.md).

---

*Because every language starts as a meme, but only the real ones ship.*
