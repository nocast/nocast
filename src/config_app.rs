use crate::generic_types::Config;
use crate::init::load_config;
use eframe;
use std::fs;
use toml;
use crate::utils::bin_path;
use crate::managepl;
use crate::ncpr;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(PartialEq)]
pub enum ConfigAppTabs {
	Style, Uninstall, Actions, Install
}

pub struct ConfigApp {
    config: Config,
    tab: ConfigAppTabs,
	plugin_search: String,
	previous_ps: String,
	plugin_results: Vec<String>,
	install_in_progress: bool,
	installing_plugin: String
}

impl Default for ConfigApp {
    fn default() -> Self {
        Self {
        	config: Config {
            	plugins: (std::collections::HashMap::new()),
        	}, 
        	tab: ConfigAppTabs::Style,
			plugin_search: String::new(),
			plugin_results: Vec::new(),
			previous_ps: String::new(),
			install_in_progress: false,
			installing_plugin: String::new()
    	}
    }
}

impl ConfigApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
        	config: load_config(),
        	..Default::default()
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
				if ui.selectable_label(self.tab == ConfigAppTabs::Install, "Install").clicked() {
                    self.tab = ConfigAppTabs::Install;
                }
                if ui.selectable_label(self.tab == ConfigAppTabs::Uninstall, "Uninstall").clicked() {
                    self.tab = ConfigAppTabs::Uninstall;
                }
                if ui.selectable_label(self.tab == ConfigAppTabs::Actions, "Actions").clicked() {
                    self.tab = ConfigAppTabs::Actions;
                }
            });
        });
        
    	let mut config_changed = false;
        //tab-specific
        egui::CentralPanel::default().show(ctx, |ui| {
			//refresh if not on actions
			if self.tab != ConfigAppTabs::Actions{
				self.config = load_config();
			}
			//installing?
			if self.install_in_progress {
                ui.label("Installing plugin...");
				ui.label("This may take a while");
				if self.config.plugins.clone().contains_key(&self.installing_plugin){
					self.install_in_progress = false;
					ui.label("Plugin installed successfully!");
				}
				return;
			}
        	match self.tab{
            	ConfigAppTabs::Style => {
            	    //style
            	}
				ConfigAppTabs::Install => {
            	    //install
					ui.label("Here you can search and install plugins");
					ui.label("If you try to install an already-installed plugin, it will update to the latest version");
					ui.separator();
					ui.add(
    					egui::TextEdit::singleline(&mut self.plugin_search)
				        	.hint_text("Start typing to search for plugins..."),
					);
					//only search if keyword has changed
					let mut results: Vec<String> = Vec::new();
					if &self.plugin_search == &self.previous_ps{
						//use results from
						results = self.plugin_results.clone();
					}
					else{
						self.previous_ps = self.plugin_search.clone();
						if !self.plugin_search.trim().is_empty() {
							results = ncpr::search_plugin(&self.plugin_search).unwrap();
							self.plugin_results = results.clone();
						}
					}
					for plugin in results.into_iter(){
						if ui.button(&plugin).clicked() {
							self.install_in_progress = true;
                    		let install_flag = Arc::new(Mutex::new((false, false))); // (in_progress, done)
                    		let install_flag_clone = install_flag.clone();
							config_changed = true;
							self.config.plugins.remove(&plugin);
							self.installing_plugin = plugin.clone();
                    		thread::spawn(move || {
                        		// Your blocking install code here
                        		managepl::install_plugin(plugin);

                        		// Signal it's done
                        		let mut state = install_flag_clone.lock().unwrap();
                        		state.0 = false;
                        		state.1 = true;
                    		});

                    		// Keep a reference so UI can poll it
                    		let install_flag_ui = install_flag.clone();
                    		self.install_in_progress = true;
                		}
            		}
				}
            	ConfigAppTabs::Uninstall => {
            	    //uninstall
					ui.label("Here you can uninstall plugins. Click on the plugin name to uninstall it.");
					ui.label("Caution!: when a plugin is clicked, it will be uninstalled without verification!");
					ui.separator();
					ui.heading("Installed plugins");
            		for (plugin, _) in self.config.plugins.clone(){
						if ui.button(&plugin).clicked() {
                    		managepl::uninstall_plugin(plugin);
                		}
            		}
					ui.separator();
            	}
            	ConfigAppTabs::Actions => {
                	ui.label("In this config page, you can turn on or off actions for plugins");
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
