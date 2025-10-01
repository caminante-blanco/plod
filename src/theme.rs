use crate::colors::*;
use eframe::egui::{self, Color32, FontData, FontDefinitions, FontFamily, Stroke, Style, Visuals};
use egui::epaint::{AlphaFromCoverage, CornerRadius, Shadow};
use egui::style::{HandleShape, NumericColorSpace, Selection, TextCursorStyle, Widgets};
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
            text_alpha_from_coverage: AlphaFromCoverage::DARK_MODE_DEFAULT,
            override_text_color: Some(LIGHT1),
            weak_text_alpha: 0.6,
            weak_text_color: None,
            widgets: Widgets {
                noninteractive: widget_visuals(DARK0, LIGHT1, DARK1),
                inactive: widget_visuals(DARK1, LIGHT1, DARK3),
                hovered: widget_visuals(DARK2, LIGHT1, LIGHT4),
                active: widget_visuals(DARK3, LIGHT1, BRIGHT_BLUE),
                open: widget_visuals(DARK2, LIGHT1, DARK3),
            },
            selection: Selection {
                bg_fill: NEUTRAL_BLUE.linear_multiply(0.5),
                stroke: Stroke::new(1.0, NEUTRAL_BLUE),
            },
            hyperlink_color: NEUTRAL_BLUE,
            faint_bg_color: DARK0_SOFT,
            extreme_bg_color: DARK0_HARD,
            text_edit_bg_color: Some(DARK0_HARD),
            code_bg_color: DARK0_HARD,
            warn_fg_color: NEUTRAL_YELLOW,
            error_fg_color: NEUTRAL_RED,
            window_corner_radius: CornerRadius::same(6),
            window_shadow: Shadow {
                offset: [10, 20],
                blur: 15,
                spread: 0,
                color: Color32::from_black_alpha(96),
            },
            window_fill: DARK0,
            window_stroke: Stroke::new(1.0, DARK3),
            window_highlight_topmost: true,
            menu_corner_radius: CornerRadius::same(6),
            panel_fill: DARK0,
            popup_shadow: Shadow {
                offset: [6, 10],
                blur: 8,
                spread: 0,
                color: Color32::from_black_alpha(96),
            },
            resize_corner_size: 12.0,
            text_cursor: TextCursorStyle {
                stroke: Stroke::new(2.0, NEUTRAL_BLUE),
                ..Default::default()
            },
            clip_rect_margin: 3.0,
            button_frame: true,
            collapsing_header_frame: false,
            indent_has_left_vline: true,
            striped: false,
            slider_trailing_fill: false,
            handle_shape: HandleShape::Rect { aspect_ratio: 0.75 },
            interact_cursor: None,
            image_loading_spinners: true,
            numeric_color_space: NumericColorSpace::GammaByte,
            disabled_alpha: 0.5,
        },
        ..Style::default()
    };
    ctx.set_style(style);
}

fn widget_visuals(
    bg_fill: Color32,
    fg_color: Color32,
    bg_stroke_color: Color32,
) -> egui::style::WidgetVisuals {
    egui::style::WidgetVisuals {
        bg_fill,
        weak_bg_fill: DARK1,
        bg_stroke: Stroke::new(1.0, bg_stroke_color),
        corner_radius: egui::CornerRadius::ZERO,
        fg_stroke: Stroke::new(0.0, fg_color),
        expansion: 0.0,
    }
}
