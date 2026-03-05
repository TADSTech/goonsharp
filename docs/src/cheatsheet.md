# GoonSharp vs Rust Cheat Sheet

Side-by-side comparison of common patterns.

## Hello World

**Rust:**
```rust
fn main() {
    println!("hello world");
}
```

**GoonSharp:**
```goons
goonsesh main() {
    goonprint!("hello world");
}
```

## Variables

**Rust:**
```rust
let x = 5;
let mut y = 10;
const MAX: i32 = 100;
```

**GoonSharp:**
```goons
goon x = 5;
goon gooning y = 10;
goon_const MAX: i32 = 100;
```

## Functions

**Rust:**
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**GoonSharp:**
```goons
goonsesh add(a: i32, b: i32) -> i32 {
    a + b
}
```

## If/Else

**Rust:**
```rust
if x > 5 {
    println!("big");
} else {
    println!("small");
}
```

**GoonSharp:**
```goons
goonif (x > 5) {
    goonprint!("big");
} goonnah {
    goonprint!("small");
}
```

## Loops

**Rust:**
```rust
for i in 0..10 { println!("{}", i); }
loop { break; }
while x > 0 { x -= 1; }
```

**GoonSharp:**
```goons
goonfor i goonin 0..10 { goonprint!("{}", i); }
goonloop { coom; }
goonwhile (x > 0) { x -= 1; }
```

## Structs

**Rust:**
```rust
struct Point { x: f64, y: f64 }
impl Point {
    fn new(x: f64, y: f64) -> Self { Self { x, y } }
}
```

**GoonSharp:**
```goons
goonstruct Point { x: f64, y: f64 }
goonimpl Point {
    goonsesh new(x: f64, y: f64) -> Self { Self { x, y } }
}
```

## Match

**Rust:**
```rust
match value {
    1 => println!("one"),
    _ => println!("other"),
}
```

**GoonSharp:**
```goons
goonmatch value {
    1 => goonprint!("one"),
    _ => goonprint!("other"),
}
```

## Booleans

**Rust:** `true` / `false`
**GoonSharp:** `edge` / `no_edge`
