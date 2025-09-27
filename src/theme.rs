use crate::colors::*;
use eframe::egui::{self, Color32, FontData, FontDefinitions, FontFamily, Stroke, Style, Visuals};
use egui::style::{Selection, Widgets};
use std::sync::Arc;

pub fn create_theme(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "HackNerdFont-Regular".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/HackNerdFont-Regular.ttf"
        ))),
    );
    fonts.font_data.insert(
        "HackNerdFont-Bold".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/HackNerdFont-Bold.ttf"
        ))),
    );
    fonts.font_data.insert(
        "HackNerdFont-Italic".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/HackNerdFont-Italic.ttf"
        ))),
    );
    fonts.font_data.insert(
        "HackNerdFont-BoldItalic".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/HackNerdFont-BoldItalic.ttf"
        ))),
    );
    fonts.font_data.insert(
        "HackNerdFontPropo-Regular".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/HackNerdFontPropo-Regular.ttf"
        ))),
    );

    fonts.font_data.insert(
        "HackNerdFontMono-Regular".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/HackNerdFontMono-Regular.ttf"
        ))),
    );
    fonts.font_data.insert(
        "HackNerdFontMono-Bold".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/HackNerdFontMono-Bold.ttf"
        ))),
    );
    fonts.font_data.insert(
        "HackNerdFontMono-Italic".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/HackNerdFontMono-Italic.ttf"
        ))),
    );
    fonts.font_data.insert(
        "HackNerdFontMono-BoldItalic".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/HackNerdFontMono-BoldItalic.ttf"
        ))),
    );

    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "HackNerdFont-Regular".to_owned());
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .push("HackNerdFont-Bold".to_owned());
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .push("HackNerdFont-Italic".to_owned());
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .push("HackNerdFont-BoldItalic".to_owned());
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .push("HackNerdFontPropo-Regular".to_owned());

    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "HackNerdFontMono-Regular".to_owned());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("HackNerdFontMono-Bold".to_owned());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("HackNerdFontMono-Italic".to_owned());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("HackNerdFontMono-BoldItalic".to_owned());

    ctx.set_fonts(fonts);

    let style = Style {
        visuals: Visuals {
            dark_mode: true,
            override_text_color: Some(FG1),
            window_fill: BG0_HARD,
            window_stroke: Stroke::NONE,

            widgets: Widgets {
                noninteractive: widget_visuals(BG1),
                inactive: widget_visuals(BG1),
                hovered: widget_visuals(BG3),
                active: widget_visuals(BG3),
                open: widget_visuals(BG3),
            },

            selection: Selection {
                bg_fill: BLUE.linear_multiply(0.5),
                stroke: Stroke::NONE,
            },

            hyperlink_color: BLUE,

            ..Visuals::dark()
        },

        ..Style::default()
    };
    ctx.set_style(style);
}

fn widget_visuals(bg_fill: Color32) -> egui::style::WidgetVisuals {
    egui::style::WidgetVisuals {
        bg_fill,
        weak_bg_fill: BG1,
        bg_stroke: Stroke::NONE,
        corner_radius: egui::CornerRadius::ZERO,
        fg_stroke: Stroke::new(0.0, FG1),
        expansion: 0.0,
    }
}
