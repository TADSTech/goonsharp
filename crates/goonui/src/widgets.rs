/// GoonUI Widgets — goon-themed UI components for desktop apps.
///
/// Every widget is designed with maximum goon energy:
/// - Purple accents everywhere
/// - Unhinged naming that makes you giggle
/// - Actually functional UI components

use egui::{Response, Ui, Widget};

use crate::theme::*;

// ─── GoonWindow ──────────────────────────────────────────────────────────────

/// A goon-themed window panel.
pub struct GoonWindow {
    title: String,
}

impl GoonWindow {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }

    pub fn show(self, ctx: &egui::Context, add_contents: impl FnOnce(&mut Ui)) {
        egui::Window::new(self.title)
            .default_size([400.0, 300.0])
            .show(ctx, |ui| {
                add_contents(ui);
            });
    }
}

// ─── GoonButton ──────────────────────────────────────────────────────────────

/// A purple goon button. Clicks with authority.
pub struct GoonButton {
    text: String,
    goon_level: GoonLevel,
}

#[derive(Clone, Copy)]
pub enum GoonLevel {
    /// Normal button
    Chill,
    /// Important button (bright purple)
    Sesh,
    /// Danger button (red)
    Ruin,
    /// Success button (green)
    Edge,
}

impl GoonButton {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            goon_level: GoonLevel::Sesh,
        }
    }

    pub fn chill(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            goon_level: GoonLevel::Chill,
        }
    }

    pub fn ruin(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            goon_level: GoonLevel::Ruin,
        }
    }

    pub fn edge(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            goon_level: GoonLevel::Edge,
        }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let color = match self.goon_level {
            GoonLevel::Chill => GOON_SURFACE,
            GoonLevel::Sesh => GOON_PURPLE,
            GoonLevel::Ruin => GOON_RED,
            GoonLevel::Edge => GOON_GREEN,
        };

        let button = egui::Button::new(
            egui::RichText::new(self.text).color(egui::Color32::WHITE),
        )
        .fill(color)
        .rounding(6.0);

        ui.add(button)
    }
}

// ─── GoonSlider ──────────────────────────────────────────────────────────────

/// A themed slider for adjusting goon intensity.
pub struct GoonSlider<'a> {
    label: String,
    value: &'a mut f64,
    range: std::ops::RangeInclusive<f64>,
}

impl<'a> GoonSlider<'a> {
    pub fn new(
        label: impl Into<String>,
        value: &'a mut f64,
        range: std::ops::RangeInclusive<f64>,
    ) -> Self {
        Self {
            label: label.into(),
            value,
            range,
        }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(self.label).color(GOON_TEXT_DIM),
            );
            ui.add(
                egui::Slider::new(self.value, self.range)
                    .text("")
            )
        })
        .inner
    }
}

// ─── GoonTextInput ───────────────────────────────────────────────────────────

/// A goon-themed text input field.
pub struct GoonTextInput<'a> {
    label: String,
    value: &'a mut String,
    hint: Option<String>,
}

impl<'a> GoonTextInput<'a> {
    pub fn new(label: impl Into<String>, value: &'a mut String) -> Self {
        Self {
            label: label.into(),
            value,
            hint: None,
        }
    }

    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(self.label).color(GOON_TEXT_DIM),
            );
            let mut edit = egui::TextEdit::singleline(self.value);
            if let Some(hint) = &self.hint {
                edit = edit.hint_text(hint.as_str());
            }
            ui.add(edit)
        })
        .inner
    }
}

// ─── GoonProgressEdge ────────────────────────────────────────────────────────

/// A progress bar showing how close to the edge you are.
pub struct GoonProgressEdge {
    progress: f32,
    label: String,
}

impl GoonProgressEdge {
    pub fn new(progress: f32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            label: format!("{}% edging", (progress * 100.0) as i32),
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        ui.add(
            egui::ProgressBar::new(self.progress)
                .text(self.label)
                .fill(if self.progress > 0.9 {
                    GOON_PINK
                } else if self.progress > 0.5 {
                    GOON_PURPLE
                } else {
                    GOON_PURPLE_DARK
                }),
        )
    }
}

// ─── GoonCoomball ────────────────────────────────────────────────────────────

/// A spinning loading indicator — the goon equivalent of a spinner.
pub struct GoonCoomball {
    size: f32,
}

impl GoonCoomball {
    pub fn new() -> Self {
        Self { size: 24.0 }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        ui.add(egui::Spinner::new().size(self.size))
    }
}

impl Default for GoonCoomball {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Extension trait for Ui ──────────────────────────────────────────────────

/// Extension methods for egui::Ui with goon flavor.
pub trait GoonUiExt {
    /// Add a goon-styled heading.
    fn goon_heading(&mut self, text: impl Into<String>);
    /// Add a goon-styled label.
    fn goon_label(&mut self, text: impl Into<String>);
    /// Add a horizontal separator with goon vibes.
    fn goon_separator(&mut self);
    /// Add a goon-styled section.
    fn goon_section(&mut self, title: impl Into<String>, add_contents: impl FnOnce(&mut Ui));
}

impl GoonUiExt for Ui {
    fn goon_heading(&mut self, text: impl Into<String>) {
        self.label(
            egui::RichText::new(text)
                .heading()
                .color(GOON_PURPLE_LIGHT),
        );
    }

    fn goon_label(&mut self, text: impl Into<String>) {
        self.label(
            egui::RichText::new(text).color(GOON_TEXT),
        );
    }

    fn goon_separator(&mut self) {
        self.add(egui::Separator::default().spacing(12.0));
    }

    fn goon_section(&mut self, title: impl Into<String>, add_contents: impl FnOnce(&mut Ui)) {
        self.group(|ui| {
            ui.label(
                egui::RichText::new(title)
                    .strong()
                    .color(GOON_PURPLE),
            );
            ui.add_space(4.0);
            add_contents(ui);
        });
    }
}
