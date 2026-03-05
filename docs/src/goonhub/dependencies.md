# Dependencies

## Adding Dependencies

```bash
goonhub add serde
goonhub add tokio --features full
```

This updates your `Goon.toml`:

```toml
[dependencies]
serde = "latest"
tokio = "latest"
```

> Note: GoonHub dependency resolution is currently basic. For complex dependency trees, you may want to manage your `Cargo.toml` directly alongside `Goon.toml`.
