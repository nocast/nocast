use eframe::egui;
use egui::{Color32, Pos2, Vec2};
use std::collections;

use crate::{ui, engine, init};
use crate::config::load_theme;
use crate::generic_types::{Item, NocastApp, Config};
use crate::win::{MAX_ITEMS, get_window};
use dark_light;

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
            theme: crate::config::load_theme(true)
        }
    }
}

impl NocastApp {
    pub fn new(cc: &eframe::CreationContext<'_>, wc: (egui::Pos2, egui::Vec2)) -> Self {
        let config = init::load_config();
        let plugins = init::load_plugins(&config);
        match dark_light::detect().expect("Error detecting theme preferences") {
        	dark_light::Mode::Dark => {
            	Self {
            		config: config,
            		plugins: plugins,
            		theme: load_theme(true),
            		wc: wc,
            		..Default::default()
        		}
        	},
        	dark_light::Mode::Light => {
            	Self {
            		config: config,
            		plugins: plugins,
            		theme: load_theme(false),
            		wc: wc,
            		..Default::default()
        		}
        	},
        	dark_light::Mode::Unspecified => {
            	Self {
            		config: config,
            		plugins: plugins,
            		theme: load_theme(true),
            		wc: wc,
            		..Default::default()
        		}
        	},
    	}
    }
}

const FONT_SIZE_DEDUCTION: f32 = 0.9;
const SCROLLING_AMOUNT_CONST: i8 = 47;

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
            } else if i.key_pressed(egui::Key::Escape) {
                std::process::exit(0);
            }
        });

		//background
        let painter = ctx.layer_painter(egui::LayerId::background());

    	let screen_rect = ctx.screen_rect();
    	painter.rect_filled(
       	 screen_rect,
       	 0.0,
       	 self.theme.WINDOW_BG
        );
        
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
                        .text_color(self.theme.INPUT_TEXT_COLOR)
                        .background_color(self.theme.INPUT_BG_COLOR)
                        .text_color_opt(Some(self.theme.INPUT_TEXT_COLOR)),
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
                                ui::render_item(panel_ui, i, self.wc.1.y, self.selecting == index, self);
                            }
                        });
                    });
            });
    }
}
