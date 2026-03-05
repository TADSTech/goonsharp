# Traits & Impl

## Traits (goontrait)

```goons
goontrait Goonable {
    goonsesh goon(&self) -> String;!

    goonsesh power_level(&self) -> i32 {
        9001  // default implementation
    }
}
```

## Implementing Traits (goonimpl)

```goons
goonstruct Chad {
    name: String,
}

goonimpl Goonable goonfor Chad {
    goonsesh goon(&self) -> String {
        goonformat!("{} is gooning hard", self.name)
    }
}
```

## Using Trait Bounds

```goons
goonsesh describe<T: Goonable>(thing: &T) {
    goonprint!("{}", thing.goon());
    goonprint!("power: {}", thing.power_level());
}
```

## Dynamic Dispatch

```goons
goonsesh take_any(g: &goon_dyn Goonable) {
    goonprint!("{}", g.goon());
}
```
