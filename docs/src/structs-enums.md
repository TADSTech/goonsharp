# Structs & Enums

## Structs (goonstruct)

```goons
goonstruct Player {
    name: String,
    health: i32,
    is_gooning: bool,
}

goonimpl Player {
    goonsesh new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            health: 100,
            is_gooning: edge,
        }
    }

    goonsesh take_damage(&gooning self, amount: i32) {
        self.health -= amount;
        goonif (self.health <= 0) {
            goonprint!("{} got ruined", self.name);
        }
    }
}
```

## Tuple Structs

```goons
goonstruct GoonId(u64);
goonstruct Point(f64, f64);
```

## Enums (goonenum)

```goons
goonenum Direction {
    Up,
    Down,
    Left,
    Right,
}

goonenum GoonResult<T, E> {
    W(T),    // Win
    L(E),    // Loss
}
```

## Matching Enums

```goons
goon dir = Direction::Up;

goonmatch dir {
    Direction::Up => goonprint!("going up"),
    Direction::Down => goonprint!("going down"),
    _ => goonprint!("sideways"),
}
```
