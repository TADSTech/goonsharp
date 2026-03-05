# Control Flow

## goonif / goonnah

```goons
goon x = 42;

goonif (x > 40) {
    goonprint!("based");
} goonnah goonif (x > 20) {
    goonprint!("mid");
} goonnah {
    goonprint!("cringe");
}
```

`goonif` is an expression:

```goons
goon label = goonif (x > 50) { "chad" } goonnah { "normie" };
```

## goonfor

```goons
goonfor i goonin 0..10 {
    goonprint!("{}", i);
}

goon items = goonvec!["a", "b", "c"];
goonfor item goonin &items {
    goonprint!("{}", item);
}
```

## goonwhile

```goons
goon gooning n = 0;
goonwhile (n < 10) {
    n += 1;
}
```

## goonloop

```goons
goon gooning count = 0;
goonloop {
    count += 1;
    goonif (count >= 5) {
        coom;  // break
    }
}
```

`goonloop` with value:

```goons
goon result = goonloop {
    goonif (some_condition()) {
        coom 42;
    }
};
```

## goonmatch

```goons
goon value = 3;

goonmatch value {
    1 => goonprint!("one"),
    2 => goonprint!("two"),
    3 => goonprint!("three"),
    _ => goonprint!("idk bro"),
}
```

See [Pattern Matching](./pattern-matching.md) for advanced patterns.
