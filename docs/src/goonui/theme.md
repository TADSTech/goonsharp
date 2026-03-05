# DarkGoon Theme

The DarkGoon theme provides a consistent purple-tinted dark aesthetic.

## Color Palette

| Name | Hex | Usage |
|------|-----|-------|
| GOON_PURPLE | `#7B2FBE` | Primary accent |
| GOON_DARK_PURPLE | `#4A1A6B` | Secondary accent |
| GOON_PINK | `#FF69B4` | Highlights |
| GOON_RED | `#FF1744` | Danger/Ruin |
| GOON_GREEN | `#00E676` | Success |
| GOON_YELLOW | `#FFEA00` | Warnings |
| GOON_BG | `#0F0A19` | Background |
| GOON_BG_LIGHT | `#1A1225` | Elevated surfaces |
| GOON_SURFACE | `#231830` | Cards/panels |
| GOON_TEXT | `#E8E0F0` | Primary text |
| GOON_TEXT_DIM | `#8B7FA0` | Secondary text |
| GOON_BORDER | `#3D2B55` | Borders |
| GOON_HOVER | `#2D1F40` | Hover states |
| GOON_SELECTION | `#4A2D6B` | Selection highlight |

## Applying the Theme

The theme is applied automatically when using `GoonApp::run()`. To apply manually:

```rust
use goonui::theme::apply_dark_goon_theme;

// Inside your eframe App::update():
apply_dark_goon_theme(&ctx);
```
