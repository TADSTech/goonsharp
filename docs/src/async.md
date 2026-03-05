# Async / Await

## Async Functions

```goons
goonasync goonsesh fetch() -> String {
    "fetched data".to_string()
}

goonasync goonsesh process() {
    goon data = fetch().goonawait;
    goonprint!("got: {}", data);
}
```

## Async Blocks

```goons
goon future = goonasync {
    goon x = expensive_computation().goonawait;
    x + 1
};
```

> Note: You'll need a Rust async runtime (like tokio) as a dependency to actually run async code. GoonSharp transpiles `goonasync` and `goonawait` to their Rust equivalents.
