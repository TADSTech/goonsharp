# Error Handling

## Result Type

GoonSharp uses Rust's `Result<T, E>` and `Option<T>` types.

```goons
goonsesh divide(a: f64, b: f64) -> Result<f64, String> {
    goonif (b == 0.0) {
        Err("division by zero bro".to_string())
    } goonnah {
        Ok(a / b)
    }
}
```

## The ? Operator

```goons
goonsesh read_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    goon n = s.parse::<i32>()?;
    Ok(n * 2)
}
```

## Option and no_goon

```goons
goonsesh find_goon(name: &str) -> Option<String> {
    goonif (name == "chad") {
        Some("found the chad".to_string())
    } goonnah {
        no_goon
    }
}
```

## ruin (panic)

For when things go catastrophically wrong:

```goons
goonsesh main() {
    goon x = -1;
    goonif (x < 0) {
        ruin("negative goon energy detected");
    }
}
```

## post_nut_clarity (debug)

Insert a debug breakpoint / diagnostic:

```goons
goonsesh main() {
    goon data = compute_stuff();
    post_nut_clarity;  // prints debug info
    process(data);
}
```
