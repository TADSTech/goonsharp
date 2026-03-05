# Hello Goon

Let's write your first GoonSharp program.

## Create a file

Create a file called `hello.goons`:

```goons
goonsesh main() {
    goonprint!("hello, goon world! 🟣");
}
```

## Run it

```bash
goonsharp hello.goons
```

You should see:

```
hello, goon world! 🟣
```

## What just happened?

1. GoonSharp **lexed** your source into tokens
2. The **parser** built an Abstract Syntax Tree (AST)
3. The **codegen** transpiled it to Rust: `fn main() { println!("hello, goon world! 🟣"); }`
4. **rustc** compiled the Rust to a native binary
5. The binary ran and printed your message

All in milliseconds. That's the goon pipeline.

## Using GoonHub

For real projects, use GoonHub:

```bash
goonhub new my_sesh
cd my_sesh
goonhub run
```

This creates a project with `Goon.toml` and `src/main.goons`, ready to go.
