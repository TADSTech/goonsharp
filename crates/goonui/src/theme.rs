/// DarkGoon Theme — the official GoonSharp color scheme.
///
/// Purple-dominant with deep backgrounds, bright accents, and
/// just the right amount of unhinged energy.

use egui::{Color32, FontId, Rounding, Stroke, TextStyle, Visuals};

/// The GoonSharp purple.
pub const GOON_PURPLE: Color32 = Color32::from_rgb(147, 51, 234);
/// Lighter purple for highlights.
pub const GOON_PURPLE_LIGHT: Color32 = Color32::from_rgb(192, 132, 252);
/// Dark purple for backgrounds.
pub const GOON_PURPLE_DARK: Color32 = Color32::from_rgb(88, 28, 135);
/// The pink accent.
pub const GOON_PINK: Color32 = Color32::from_rgb(236, 72, 153);
/// Background color.
pub const GOON_BG: Color32 = Color32::from_rgb(15, 10, 25);
/// Panel background.
pub const GOON_BG_PANEL: Color32 = Color32::from_rgb(25, 18, 40);
/// Surface color.
pub const GOON_SURFACE: Color32 = Color32::from_rgb(35, 25, 55);
/// Text color.
pub const GOON_TEXT: Color32 = Color32::from_rgb(237, 233, 254);
/// Subtle text color.
pub const GOON_TEXT_DIM: Color32 = Color32::from_rgb(139, 128, 168);
/// Success green.
pub const GOON_GREEN: Color32 = Color32::from_rgb(74, 222, 128);
/// Warning yellow.
pub const GOON_YELLOW: Color32 = Color32::from_rgb(250, 204, 21);
/// Error red.
pub const GOON_RED: Color32 = Color32::from_rgb(248, 113, 113);

/// Apply the DarkGoon theme to an egui context.
pub fn apply_dark_goon_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    let mut visuals = Visuals::dark();

    // Window
    visuals.window_fill = GOON_BG_PANEL;
    visuals.window_stroke = Stroke::new(1.0, GOON_PURPLE_DARK);
    visuals.window_rounding = Rounding::same(8.0);

    // Panel
    visuals.panel_fill = GOON_BG;

    // Widgets
    visuals.widgets.noninteractive.bg_fill = GOON_SURFACE;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, GOON_TEXT);
    visuals.widgets.noninteractive.rounding = Rounding::same(4.0);

    visuals.widgets.inactive.bg_fill = GOON_SURFACE;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, GOON_TEXT_DIM);
    visuals.widgets.inactive.rounding = Rounding::same(4.0);

    visuals.widgets.hovered.bg_fill = GOON_PURPLE_DARK;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, GOON_TEXT);
    visuals.widgets.hovered.rounding = Rounding::same(4.0);

    visuals.widgets.active.bg_fill = GOON_PURPLE;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.widgets.active.rounding = Rounding::same(4.0);

    // Selection
    visuals.selection.bg_fill = GOON_PURPLE.linear_multiply(0.4);
    visuals.selection.stroke = Stroke::new(1.0, GOON_PURPLE_LIGHT);

    // Override text color
    visuals.override_text_color = Some(GOON_TEXT);

    style.visuals = visuals;

    // Text styles
    style.text_styles.insert(
        TextStyle::Heading,
        FontId::proportional(24.0),
    );
    style.text_styles.insert(
        TextStyle::Body,
        FontId::proportional(14.0),
    );
    style.text_styles.insert(
        TextStyle::Monospace,
        FontId::monospace(13.0),
    );
    style.text_styles.insert(
        TextStyle::Button,
        FontId::proportional(14.0),
    );
    style.text_styles.insert(
        TextStyle::Small,
        FontId::proportional(11.0),
    );

    // Spacing
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);
    style.spacing.window_margin = egui::Margin::same(12.0);

    ctx.set_style(style);
}
