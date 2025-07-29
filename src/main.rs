mod plugins;
mod config;
mod engine;
mod generic_types;
mod init;
mod monitor;
mod ui;
mod ncpr;
mod utils;
mod win;
mod config_app;
mod app;
mod managepl;
mod setup;
use crate::app::start_app;
use std::env;
use crate::utils::bin_path;
use std::fs;
use crate::setup::setup;

fn main(){
    let args: Vec<String> = env::args().collect();
    if (args.len() < 2){
    	start_app();
	}
    else if args[1] == "setup" {
        setup();
    }
    else if args[1] == "config" {
        crate::config_app::config_app();
    }
    else if args[1] == "list"{
		crate::managepl::list_plugins();
    }
    else if args[1] == "install"{
        if args.len() > 2 {
    		crate::managepl::install_plugin(args[2].clone());
		}
        else{
            eprintln!("Missing parameter when running 'install': 'plugin_name' not given");
        }
    }
    else if args[1] == "uninstall"{
    	if args.len() > 2 {
    		crate::managepl::uninstall_plugin(args[2].clone());
		}
        else{
            eprintln!("Missing parameter when running 'uninstall': 'plugin_name' not given");
        }
    }
    else{
        eprintln!("Unknown keyword '{}'", args[1]);
    }
}
