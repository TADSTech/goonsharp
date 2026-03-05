# Pattern Matching

## Basic Matching

```goons
goon x = 42;
goonmatch x {
    0 => goonprint!("zero"),
    1..=10 => goonprint!("small"),
    _ => goonprint!("big"),
}
```

## Destructuring

```goons
goon point = (3, 7);
goonmatch point {
    (0, 0) => goonprint!("origin"),
    (x, 0) => goonprint!("on x-axis at {}", x),
    (0, y) => goonprint!("on y-axis at {}", y),
    (x, y) => goonprint!("at ({}, {})", x, y),
}
```

## Struct Patterns

```goons
goonstruct Goon {
    name: String,
    level: i32,
}

goon g = Goon { name: "Chad".to_string(), level: 99 };

goonmatch g {
    Goon { level: 99, .. } => goonprint!("max level goon"),
    Goon { name, level } => goonprint!("{} at level {}", name, level),
}
```

## Guards

```goons
goonmatch x {
    n goonif (n < 0) => goonprint!("negative"),
    n goonif (n > 100) => goonprint!("huge"),
    n => goonprint!("normal: {}", n),
}
```

## Or Patterns

```goons
goonmatch x {
    1 | 2 | 3 => goonprint!("one two or three"),
    _ => goonprint!("something else"),
}
```
