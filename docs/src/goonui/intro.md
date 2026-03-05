# Getting Started with GoonUI

GoonUI is a desktop GUI framework for GoonSharp, built on top of [egui](https://github.com/emilk/egui).

## Add GoonUI to Your Project

In your `Goon.toml`:

```toml
[dependencies]
goonui = { path = "../../crates/goonui" }
```

## Your First Window

```rust
use goonui::prelude::*;

fn main() {
    GoonApp::run("My Goon App", |ctx| {
        GoonWindow::new("Main Window").show(ctx, |ui| {
            ui.goon_heading("Welcome to GoonUI");
            ui.goon_label("This is a goon-themed window");
            ui.goon_separator();

            if GoonButton::new("Click me", GoonLevel::Sesh).show(ui).clicked() {
                println!("clicked!");
            }
        });
    });
}
```

## DarkGoon Theme

GoonUI automatically applies the DarkGoon theme — deep purple backgrounds with neon accents. See the [theme reference](./theme.md) for colors.

## Available Widgets

- `GoonWindow` — themed window container
- `GoonButton` — button with GoonLevel intensity
- `GoonSlider` — purple-accented slider
- `GoonTextInput` — styled text input
- `GoonProgressEdge` — progress bar that changes color
- `GoonCoomball` — animated spinner

See [Widgets Reference](./widgets.md) for full details.
