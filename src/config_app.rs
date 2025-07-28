use crate::generic_types::Config;
use crate::init::load_config;
use eframe;
use std::fs;
use toml;
use crate::utils::bin_path;

#[derive(PartialEq)]
pub enum ConfigAppTabs {
	Style, Plugins
}

pub struct ConfigApp {
    config: Config,
    tab: ConfigAppTabs
}

impl Default for ConfigApp {
    fn default() -> Self {
        Self {
        	config: Config {
            	plugins: (std::collections::HashMap::new()),
        	}, 
        	tab: ConfigAppTabs::Style
    	}
    }
}

impl ConfigApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
        	config: load_config(),
        	tab: ConfigAppTabs::Style
    	}
    }
}

pub fn config_app() {
    let ops = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "NoCast configuration",
        ops,
        Box::new(|cc| Ok(Box::new(ConfigApp::new(cc)))),
    );
}

pub fn update_config(cont: &mut ConfigApp){
    fs::write(bin_path() + "/nocast.toml", toml::to_string_pretty(&cont.config).expect("Could not generate config file")).expect("Could not write config file");
}

impl eframe::App for ConfigApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        //tabs
        egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(self.tab == ConfigAppTabs::Style, "Style").clicked() {
                    self.tab = ConfigAppTabs::Style;
                }
                if ui.selectable_label(self.tab == ConfigAppTabs::Plugins, "Plugins").clicked() {
                    self.tab = ConfigAppTabs::Plugins;
                }
            });
        });
        
    	let mut config_changed = false;
        //tab-specific
        egui::CentralPanel::default().show(ctx, |ui| {
        	match self.tab{
            	ConfigAppTabs::Style => {
            	    //style
            	}
            	ConfigAppTabs::Plugins => {
                	ui.label("In this config page, you can turn on or off actions for plugins");
            	    //plugins
            	    for (plugin, (path, actions)) in self.config.plugins.iter_mut(){
            	        ui.heading(plugin.clone());
            			for (action, enabled) in actions.iter_mut() {
                			let response = ui.checkbox(enabled, action);
                			if response.changed() {
                    			config_changed = true;	
               		 	}
           			 }
           			 ui.separator();
            	    }
            	}
       	 }
		});
        if config_changed{
        	update_config(self);
    	}
    }
}
