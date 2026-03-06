/// # GoonUI — Desktop UI framework for GoonSharp
///
/// Built on top of **egui** (immediate-mode GUI), GoonUI provides a set of
/// goon-themed widgets and the "DarkGoon" theme for building desktop applications
/// that look like they were made during a 12-hour session.
///
/// ## Quick Start
/// ```rust,ignore
/// use goonui::prelude::*;
///
/// fn main() -> eframe::Result<()> {
///     GoonApp::run("My Goon App", |ctx, ui| {
///         GoonWindow::new("sesh panel").show(ctx, |ui| {
///             ui.goon_heading("welcome to the goon zone");
///             if GoonButton::new("click to goon").show(ui).clicked() {
///                 println!("gooned!");
///             }
///             GoonSlider::new("intensity", &mut 69.0, 0.0..=100.0).show(ui);
///         });
///     })
/// }
/// ```

pub mod theme;
pub mod widgets;

pub mod prelude {
    pub use crate::theme::*;
    pub use crate::widgets::*;
    pub use eframe;
    pub use egui;
}

/// The main GoonSharp application wrapper.
pub struct GoonApp;

impl GoonApp {
    /// Run a GoonUI application with the DarkGoon theme.
    pub fn run(
        title: &str,
        app_fn: fn(&egui::Context, &mut eframe::Frame),
    ) -> eframe::Result<()> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([800.0, 600.0])
                .with_title(title),
            ..Default::default()
        };

        let title = title.to_string();
        eframe::run_simple_native(&title, options, move |ctx, frame| {
            theme::apply_dark_goon_theme(ctx);
            app_fn(ctx, frame);
        })
    }
}
