# Variables & Types

## Bindings

```goons
// Immutable
goon x = 42;
goon name: String = "goon".to_string();

// Mutable
goon gooning counter = 0;
counter += 1;

// Constants
goon_const PI: f64 = 3.14159;
```

## Primitive Types

GoonSharp uses standard Rust types: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `f32`, `f64`, `bool`, `char`, `&str`, `String`.

Boolean values use goon names:
- `edge` → `true`
- `no_edge` → `false`

## Type Inference

```goons
goon x = 42;           // i32
goon y = 3.14;         // f64
goon z = "hello";      // &str
goon b = edge;         // bool
```

## References

```goons
goon x = 42;
goon r = &x;           // &i32
goon gooning m = 10;
goon mr = &gooning m;  // &mut i32
```

## Tuples

```goons
goon pair = (1, "hello");
goon (a, b) = pair;
goonprint!("{} {}", a, b);
```

## Arrays and Vectors

```goons
goon arr = [1, 2, 3, 4, 5];
goon v = goonvec![1, 2, 3];
```
