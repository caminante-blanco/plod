use crate::colors::*;
use crate::dev_icons;
use crate::theme::create_theme;
use crate::tree_view_ui;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct PortfolioApp {}

impl PortfolioApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        create_theme(&cc.egui_ctx);

        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for PortfolioApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.label("NORMAL");
        });

        egui::SidePanel::left("file_tree")
            .default_width(200.0)
            .min_width(50.0)
            .show(ctx, |ui| {
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let icon_info = dev_icons::get_icon_for_folder(false);
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(icon_info.icon).color(icon_info.color),
                            )
                            .selectable(false),
                        );
                        ui.heading("~/website");
                    });
                    ui.separator();

                    tree_view_ui::show(ui, ui.make_persistent_id("website_tree_id"), |builder| {
                        builder.dir(0, "Personal");
                        builder.leaf(1, "about-me.md");
                        builder.close_dir();

                        builder.dir(2, "Projects");
                        builder.dir(3, "This_website");
                        builder.leaf(4, "building-without-js.md");
                        builder.close_dir();
                        builder.leaf(5, "merge-perser.md");
                        builder.close_dir();

                        builder.dir(6, "Plod");
                        builder.leaf(7, "first-post.md");
                        builder.leaf(8, "getting-started-with-rust.md");
                        builder.close_dir()
                    });
                });
                let rect = ui.clip_rect();
                // Draw the line on the panel's right edge
                ui.painter()
                    .vline(rect.right(), rect.y_range(), egui::Stroke::new(5.0, BG3));
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to Plod");
            ui.label("Central panel test");
        });
    }
}
