# Your First Goonsesh

A `goonsesh` is GoonSharp for "function". Let's build something real.

## Variables

```goons
goonsesh main() {
    // immutable binding
    goon name = "Big Goon";

    // mutable binding
    goon gooning age = 25;
    age = 26;

    // constant
    goon_const MAX_GOONS: i32 = 420;

    goonprint!("{} is {} and max is {}", name, age, MAX_GOONS);
}
```

## Functions

```goons
goonsesh add(a: i32, b: i32) -> i32 {
    a + b
}

goonsesh greet(name: &str) {
    goonprint!("what's good, {}!", name);
}

goonsesh main() {
    goon result = add(69, 351);
    greet("fam");
    goonprint!("sum: {}", result);
}
```

## Control Flow

```goons
goonsesh main() {
    goon x = 42;

    goonif (x > 40) {
        goonprint!("based");
    } goonnah {
        goonprint!("cringe");
    }

    goonfor i goonin 0..5 {
        goonprint!("rep {}", i);
    }

    goon gooning count = 0;
    goonloop {
        count += 1;
        goonif (count >= 3) {
            coom;  // break
        }
    }
}
```

## Structs

```goons
goonstruct Goon {
    name: String,
    power_level: i32,
    is_gooning: bool,
}

goonimpl Goon {
    goonsesh new(name: String, power: i32) -> Self {
        Self {
            name,
            power_level: power,
            is_gooning: edge,
        }
    }

    goonsesh flex(&self) {
        goonprint!("{} flexes with power {}", self.name, self.power_level);
    }
}

goonsesh main() {
    goon g = Goon::new("Chad".to_string(), 9001);
    g.flex();
}
```

## What's Next?

- [Keywords reference](./keywords.md) for the full keyword list
- [Pattern matching](./pattern-matching.md) for goon-flavored `match`
- [Error handling](./error-handling.md) for Result and Option with goon names
