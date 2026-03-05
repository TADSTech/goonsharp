# Modules & Imports

## Modules

```goons
goonmod utils {
    goonpub goonsesh helper() {
        goonprint!("helping");
    }
}

goonsesh main() {
    utils::helper();
}
```

## Imports

```goons
goonuse std::collections::HashMap;
goonuse std::io::{self, Read, Write};
```

## Re-exports

```goons
goonpub goonuse crate::utils::helper;
```
