use egui::{self, Color32};
use crate::generic_types::Item;
use crate::config::*;
use egui::{RichText, TextStyle, WidgetText, Label};

pub fn render_item(ui: &mut egui::Ui, i: Item, height: f32, selected: bool, app: &mut crate::generic_types::NocastApp) -> egui::Response {
    ui.set_min_height(height);

    let bg_color = if selected {
        app.theme.SELECTED_ITEM_COLOR
    } else {
        app.theme.NORMAL_ITEM_COLOR
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
                ui.add(
                	Label::new(
    					WidgetText::from(
        					egui::RichText::new(&i.title)
                    		//.font(egui::FontId { size: height, family: egui::FontFamily::Monospace})
                    		.color(app.theme.ITEM_TITLE_COLOR)
    					)
					)
    				.truncate(),
                );
                ui.add(
                	Label::new(
    					WidgetText::from(
        				 egui::RichText::new(&i.description)
                       	 .small()
                      	  .color(app.theme.ITEM_DESC_COLOR)
						)
					)
    				.truncate(),
                );
    	   });

            // Right-align keyboard shortcut
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                //nothing, just for the rect to take full width
            });
        });
    }).response
}