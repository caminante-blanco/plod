use egui::text::LayoutJob;
use egui::{Area, Color32, FontId, Layout, TextFormat};

use crate::colors::*;
use crate::colors::*;
use crate::theme::create_theme;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AppConfig {
    pub welcome_on_startup: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            welcome_on_startup: true,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct AppState {
    pub vim_mode: VimMode,
    pub command_history: Vec<String>,
}

pub struct TransientState {
    pub top_level_state: TopLevelState,
    pub is_running: bool,
    pub command_buffer: String,
    pub key_buffer: String,
}

impl Default for TransientState {
    fn default() -> Self {
        Self {
            top_level_state: TopLevelState::Welcome,
            is_running: true,
            command_buffer: String::new(),
            key_buffer: String::new(),
        }
    }
}

pub enum TopLevelState {
    Welcome,
    Main,
}

impl Default for TopLevelState {
    fn default() -> Self {
        Self::Welcome
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default, PartialEq, Eq, Clone, Copy)]
pub enum VimMode {
    #[default]
    Normal,
    Insert,
    Visual,
    Command,
}

enum WelcomeLine {
    Text(String),
    URL(String),
    Command {
        command: egui::text::LayoutJob,
        description: String,
    },
    Blank,
}

#[derive(Default)]
pub struct PortfolioApp {
    config: AppConfig,
    state: AppState,
    transient: TransientState,
}

impl PortfolioApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let config: AppConfig = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, "config"))
            .unwrap_or_default();

        let state: AppState = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, "state"))
            .unwrap_or_default();

        create_theme(&cc.egui_ctx);

        let mut app = Self {
            config,
            state,
            transient: TransientState::default(),
        };

        if !app.config.welcome_on_startup {
            app.transient.top_level_state = TopLevelState::Main;
        }
        app
    }
    fn welcome_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut should_transition = false;
            if ui.input(|i| i.pointer.any_click()) {
                should_transition = true;
            }

            ctx.input(|i| {
                for event in &i.events {
                    if let egui::Event::Text(text) = event {
                        if text == "i" {
                            should_transition = true;
                        }
                    }
                }
            });

            if should_transition {
                self.transient.top_level_state = TopLevelState::Main;
            }
            Area::new("welcome-string".into())
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        let font_id = egui::FontId::monospace(16.0);

                        let wrap_width = ui.fonts(|f| {
                            f.layout_no_wrap("W".repeat(46), font_id.clone(), Color32::BLACK)
                                .size()
                                .x
                        });
                        //ui.set_max_width(wrap_width);

                        let lines = get_welcome_lines();

                        for line in lines {
                            match line {
                                WelcomeLine::Text(text) => {
                                    ui.label(egui::RichText::new(text).font(font_id.clone()));
                                }
                                WelcomeLine::URL(url) => {
                                    let styled_url =
                                        egui::RichText::new(&url).font(font_id.clone());
                                    ui.hyperlink_to(styled_url, url);
                                }
                                WelcomeLine::Blank => {
                                    ui.add_space(15.0);
                                }
                                WelcomeLine::Command {
                                    command,
                                    description,
                                } => {
                                    ui.scope(|ui| {
                                        ui.with_layout(
                                            Layout::left_to_right(egui::Align::Center),
                                            |ui| ui.label(command),
                                        );
                                        ui.with_layout(
                                            Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.label(
                                                    egui::RichText::new(&description)
                                                        .font(FontId::monospace(16.0)),
                                                )
                                            },
                                        );
                                    });
                                }
                            };
                        }
                    })
                });
        });
    }

    fn handle_input(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}

    fn main_ui(&mut self, _ctx: &egui::Context) {}
}

impl eframe::App for PortfolioApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "config", &self.config);
        eframe::set_value(storage, "state", &self.state);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.transient.top_level_state {
            TopLevelState::Welcome => {
                self.welcome_ui(ctx);
            }
            TopLevelState::Main => {
                self.handle_input(ctx, frame);
                self.main_ui(ctx);
            }
        }
    }
}

fn create_layout_job(fragments: Vec<(&str, Color32)>) -> LayoutJob {
    let mut job = LayoutJob::default();
    //Ok, the font and color shouldnt be hardoced here, but im working with what ive got ok
    for (text, color) in fragments {
        job.append(
            text,
            0.0,
            TextFormat {
                font_id: FontId::monospace(16.0),
                color,
                ..Default::default()
            },
        );
    }
    job
}

fn get_welcome_lines() -> Vec<WelcomeLine> {
    vec![
        WelcomeLine::Text("WALKER WHITE".into()),
        WelcomeLine::Blank,
        WelcomeLine::Text(
            "I am a computational linguist with a love for syntax and parsers".into(),
        ),
        WelcomeLine::Blank,
        WelcomeLine::URL("https://www.linkedin.com/in/walkercwhite".into()),
        WelcomeLine::URL("https://github.com/caminante-blanco".into()),
        WelcomeLine::Blank,
        WelcomeLine::Command {
            command: create_layout_job(vec![("type i", LIGHT1), ("<ENTER>", LIGHT4)]),
            description: "to see the rest of the site!".into(),
        },
        WelcomeLine::Command {
            command: create_layout_job(vec![("type :help", LIGHT1), ("<ENTER>", LIGHT4)]),
            //command: "type :help<Enter>".into(),
            description: "If you've never used Vim before".into(),
        },
        WelcomeLine::Command {
            command: create_layout_job(vec![
                ("type ", LIGHT1),
                ("<LEADER>", LIGHT4),
                ("ee", LIGHT1),
            ]),
            //command: "type  <leader>ee".into(),
            description: "to move to the file-picker".into(),
        },
        WelcomeLine::Command {
            command: create_layout_job(vec![
                ("type ", LIGHT1),
                ("<LEADER>", LIGHT4),
                ("ef", LIGHT1),
            ]),
            //command: "type  <leader>ef".into(),
            description: "to move back to the content".into(),
        },
    ]
}

//  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//      egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
//          ui.label("NORMAL");
//      });

//      egui::SidePanel::left("file_tree")
//          .default_width(200.0)
//          .min_width(50.0)
//          .show(ctx, |ui| {
//              egui::ScrollArea::horizontal().show(ui, |ui| {
//                  ui.horizontal(|ui| {
//                      let icon_info = dev_icons::get_icon_for_folder(false);
//                      ui.add(
//                          egui::Label::new(
//                              egui::RichText::new(icon_info.icon).color(icon_info.color),
//                          )
//                          .selectable(false),
//                      );
//                      ui.heading("~/website");
//                  });
//                  ui.separator();

//                  tree_view_ui::show(ui, ui.make_persistent_id("website_tree_id"), |builder| {
//                      builder.dir(0, "Personal");
//                      builder.leaf(1, "about-me.md");
//                      builder.close_dir();

//                      builder.dir(2, "Projects");
//                      builder.dir(3, "This_website");
//                      builder.leaf(4, "building-without-js.md");
//                      builder.close_dir();
//                      builder.leaf(5, "merge-perser.md");
//                      builder.close_dir();

//                      builder.dir(6, "Plod");
//                      builder.leaf(7, "first-post.md");
//                      builder.leaf(8, "getting-started-with-rust.md");
//                      builder.close_dir()
//                  });
//              });
//              let rect = ui.clip_rect();
//              // Draw the line on the panel's right edge
//              ui.painter()
//                  .vline(rect.right(), rect.y_range(), egui::Stroke::new(5.0, BG3));
//          });

//      egui::CentralPanel::default().show(ctx, |ui| {
//          ui.heading("Welcome to Plod");
//          ui.label("Central panel test");
//      });
//  }
//}
