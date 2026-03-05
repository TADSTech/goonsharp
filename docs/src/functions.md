# Functions (goonsesh)

In GoonSharp, functions are declared with `goonsesh`.

## Basic Functions

```goons
goonsesh greet() {
    goonprint!("what's good");
}

goonsesh add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Visibility

```goons
goonpub goonsesh public_function() {
    goonprint!("everyone can see this");
}
```

## Generic Functions

```goons
goonsesh identity<T>(x: T) -> T {
    x
}
```

## Async Functions

```goons
goonasync goonsesh fetch_data() -> String {
    // async work here
    "data".to_string()
}
```

## Closures

```goons
goon add = |a, b| a + b;
goon result = add(2, 3);

goon numbers = goonvec![1, 2, 3, 4, 5];
goon evens: Vec<_> = numbers.iter().filter(|n| **n % 2 == 0).collect();
```
