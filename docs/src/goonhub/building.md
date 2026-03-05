# Building & Running

## Build

```bash
goonhub build
```

This:
1. Reads `Goon.toml`
2. Compiles all `.goons` files to Rust
3. Invokes `rustc` to produce a binary

## Run

```bash
goonhub run
```

Build and execute in one step.

## Test

```bash
goonhub test
```

Runs test functions (coming soon: native goon test macros).

## Clean

```bash
goonhub clean
```

Removes build artifacts.

## REPL (goon mode)

```bash
goonhub goon
```

Interactive REPL — type GoonSharp expressions and see the transpiled Rust + output:

```
🟣 GoonSharp REPL v69.0.0
Type GoonSharp expressions. Type 'quit' to exit.
goon> goon x = 42;
→ let x = 42;
goon> goonprint!("{}", x);
→ println!("{}", x);
42
```
