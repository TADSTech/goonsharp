# GoonUI Widgets Reference

## GoonButton

Buttons come in 4 intensity levels:

```rust
GoonButton::new("Chill", GoonLevel::Chill)    // subtle, gray
GoonButton::new("Sesh", GoonLevel::Sesh)      // normal, purple
GoonButton::new("Edge", GoonLevel::Edge)       // intense, pink
GoonButton::new("Ruin", GoonLevel::Ruin)       // DANGER, red
```

## GoonSlider

```rust
GoonSlider::new(&mut value, 0.0..=100.0).show(ui);
```

## GoonTextInput

```rust
GoonTextInput::new(&mut text).hint("type here...").show(ui);
```

## GoonProgressEdge

Progress bar that transitions through colors:
- 0–50%: Purple (chillin')
- 50–80%: Pink (edging)
- 80–100%: Red (about to coom)

```rust
GoonProgressEdge::new(0.75).show(ui);
```

## GoonCoomball

Animated spinner for loading states:

```rust
GoonCoomball::new().show(ui);
```

## GoonUiExt Trait

Extension methods on `egui::Ui`:

```rust
ui.goon_heading("Title");
ui.goon_label("Body text");
ui.goon_separator();
ui.goon_section("Section Name", |ui| {
    ui.label("Section content");
});
```
