# Closures

Closures in GoonSharp use standard Rust closure syntax.

```goons
goon add = |a: i32, b: i32| -> i32 { a + b };
goon double = |x| x * 2;
```

## Move Closures

```goons
goon name = "goon".to_string();
goon greet = goon_move || {
    goonprint!("hello {}", name);
};
greet();
```

## As Arguments

```goons
goonsesh apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

goonsesh main() {
    goon result = apply(|x| x * x, 5);
    goonprint!("{}", result); // 25
}
```

## Iterator Chains

```goons
goon nums = goonvec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

goon sum: i32 = nums.iter()
    .filter(|n| **n % 2 == 0)
    .map(|n| n * n)
    .sum();

goonprint!("sum of even squares: {}", sum);
```
