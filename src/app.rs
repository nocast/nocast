use eframe::egui;
use egui::{Color32, Pos2, Vec2};
use std::collections;

use crate::{ui, engine, init};
use crate::config::*;
use crate::generic_types::{Item, NocastApp, Config};
use crate::win::{MAX_ITEMS, get_window};

impl Default for NocastApp {
    fn default() -> Self {
        Self {
            previous_query: String::from("none"),
            current_items: Vec::new(),
            query: String::new(),
            wc: (egui::Pos2::new(0.0, 0.0), egui::Vec2::new(0.0, 0.0)),
            selecting: 0,
            plugins: Vec::new(),
            config: Config {
                plugins: (std::collections::HashMap::new()),
            },
        }
    }
}

impl NocastApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, wc: (egui::Pos2, egui::Vec2)) -> Self {
        let config = init::load_config();
        let plugins = init::load_plugins(&config);
        Self {
            config: config,
            plugins: plugins,
            wc: wc,
            ..Default::default()
        }
    }
}

const FONT_SIZE_DEDUCTION: f32 = 0.9;
const SCROLLING_AMOUNT_CONST: i8 = 42;

pub fn start_app() {
    let window_const = get_window();
    let ops = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            decorations: Some(false),
            // titlebar_shown: Some(false),
            resizable: Some(false),
            transparent: Some(true),
            position: Some(window_const.0),
            inner_size: Some(Vec2::new(
                window_const.1.x,
                window_const.1.y * (MAX_ITEMS + 1.0),
            )),
            ..Default::default()
        },
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Nocastpb - plugin-based search bar",
        ops,
        Box::new(|cc| Ok(Box::new(NocastApp::new(cc, window_const)))),
    );
}

impl eframe::App for NocastApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        //style
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill =
            egui::Color32::from_rgba_premultiplied(WINDOW_BG, WINDOW_BG, WINDOW_BG, WINDOW_OPACITY); // Semi-dark glass
        style.visuals.panel_fill =
            egui::Color32::from_rgba_premultiplied(WINDOW_BG, WINDOW_BG, WINDOW_BG, WINDOW_OPACITY); // More transparent
        style.visuals.window_stroke =
            egui::Stroke::new(WINDOW_STROKE, egui::Color32::from_white_alpha(40));
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_white_alpha(20);
        // Text and element styling
        style.visuals.override_text_color = Some(egui::Color32::WHITE);
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_white_alpha(40);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_white_alpha(60);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_white_alpha(80);
        ctx.set_style(style);
        //Instantiate font
        let mut fonts = egui::FontDefinitions::default();
        // Install my own font (maybe supporting non-latin characters):
        fonts.font_data.insert(
            "jetbrains".to_owned(),
            std::sync::Arc::new(
                // .ttf and .otf supported
                egui::FontData::from_static(include_bytes!("../jetbrains.ttf")),
            ),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "jetbrains".to_owned());
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "jetbrains".to_owned());
        ctx.set_fonts(fonts);

        //navigation
        ctx.input_mut(|i| {
            if i.key_pressed(egui::Key::ArrowUp) {
                self.selecting -= 1;
                i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp);
            } else if i.key_pressed(egui::Key::ArrowDown) {
                self.selecting += 1;
                i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown);
            } else if i.key_pressed(egui::Key::Enter) {
                engine::run_item_at(self.selecting, self);
                i.consume_key(egui::Modifiers::NONE, egui::Key::Enter);
            }
        });

        //render
        egui::CentralPanel::default()
            .frame(egui::Frame {
                inner_margin: egui::Margin::ZERO, // Critical: Remove window padding
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Monospace,
                    egui::FontId::new(
                        self.wc.1.y * FONT_SIZE_DEDUCTION,
                        egui::FontFamily::Monospace,
                    ),
                );
                let input_item = ui.add_sized(
                    Vec2::new(self.wc.1.x, self.wc.1.y),
                    egui::TextEdit::singleline(&mut self.query)
                        .hint_text("Search...")
                        .font(egui::TextStyle::Monospace)
                        .text_color(Color32::from_rgba_premultiplied(
                            INPUT_TEXT_COLOR,
                            INPUT_TEXT_COLOR,
                            INPUT_TEXT_COLOR,
                            INPUT_OPACITY,
                        ))
                        .background_color(Color32::from_rgba_premultiplied(
                            INPUT_BG_COLOR,
                            INPUT_BG_COLOR,
                            INPUT_BG_COLOR,
                            INPUT_OPACITY,
                        ))
                        .text_color_opt(Some(Color32::from_rgba_premultiplied(
                            INPUT_TEXT_COLOR,
                            INPUT_TEXT_COLOR,
                            INPUT_TEXT_COLOR,
                            INPUT_OPACITY,
                        ))),
                );
                input_item.request_focus();

                egui::ScrollArea::vertical()
                    .scroll_offset(Vec2::new(
                        0.0,
                        f32::from(self.selecting) * f32::from(SCROLLING_AMOUNT_CONST),
                    ))
                    .max_height(self.wc.1.y * MAX_ITEMS)
                    .show(ui, |scroll_ui| {
                        let items: Vec<Item> = if self.query != self.previous_query {
                            self.previous_query = self.query.clone();
                            engine::query_items(&(self))
                        } else {
                            self.current_items.clone()
                        };
                        self.current_items = items.clone();

                        // tidy "self.selecting"
                        let items_n: i16 = items.len().try_into().unwrap();
                        if items_n != 0 {
                            self.selecting = self.selecting;
                        }
                        if self.selecting < 0 {
                            self.selecting = items_n - 1;
                        } else if self.selecting > items_n - 1 {
                            self.selecting = 0;
                        }
                        //
                        let mut index = -1;
                        scroll_ui.vertical(|panel_ui| {
                            for i in items {
                                index += 1;
                                ui::render_item(panel_ui, i, self.wc.1.y, self.selecting == index);
                            }
                        });
                    });
            });
    }
}
