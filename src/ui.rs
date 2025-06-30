use egui::{self, Color32};
use crate::generic_types::Item;
use crate::config::*;

pub fn render_item(ui: &mut egui::Ui, i: Item, height: f32, selected: bool) -> egui::Response {
    ui.set_min_height(height);

    let bg_color = if selected {
        egui::Color32::from_rgba_premultiplied(ITEM_WHITE, ITEM_WHITE, ITEM_WHITE, ITEM_OPACITY)
    } else {
        egui::Color32::from_rgba_premultiplied(ITEM_LIGHT_GRAY, ITEM_LIGHT_GRAY, ITEM_LIGHT_GRAY, ITEM_OPACITY)
    };

    // Use a frame with exact height allocation
    let frame = egui::Frame::new()
        .fill(bg_color)
        .inner_margin(egui::Margin::symmetric(10i8, 5i8))
        .corner_radius(4.0);

    frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            // Icon (fixed size)
            let icon_size = egui::Vec2::splat(height * 0.8);
        /*ui.add(egui::Image::new(/* your texture */)
                .rounding(4.0)
                .tint(egui::Color32::LIGHT_BLUE)
                .fit_to_exact_size(icon_size)
            );*/

            
            // Text container (expands horizontally)
            ui.vertical(|ui| {
                ui.add_space(2.0); // Vertical centering adjustment
                ui.label(
                    egui::RichText::new(&i.title)
                        //.font(egui::FontId { size: height, family: egui::FontFamily::Monospace})
                        .color(Color32::from_rgba_premultiplied(ITEM_TITLE_COLOR, ITEM_TITLE_COLOR, ITEM_TITLE_COLOR, ITEM_OPACITY))
                );
                ui.label(
                    egui::RichText::new(&i.description)
                        .small()
                        .color(Color32::from_rgba_premultiplied(ITEM_DESC_COLOR, ITEM_DESC_COLOR, ITEM_DESC_COLOR, ITEM_OPACITY))
                );
            });

            // Right-align keyboard shortcut
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("Ctrl+A")
                        .small()
                        .weak()
                );
            });
        });
    }).response
}