mod plugins;
mod config;
mod engine;
mod generic_types;
mod init;
mod monitor;
mod ui;
mod utils;
mod win;
mod app;
mod managepl;
use crate::app::start_app;
use std::env;
use crate::utils::bin_path;
use std::fs;

fn setup(){
    println!("- NOCAST setup");
    
    // clear existing setup
    println!("Cleaning possible traces of other setups...");
    if std::path::Path::new(&(bin_path() + "/nocast.toml")).is_file(){
        fs::remove_file(bin_path() + "/nocast.toml").expect("Could not remove existing config file");
    }
    if std::path::Path::new(&(bin_path() + "/plugins/")).is_dir(){
        fs::remove_dir_all(bin_path() + "/plugins/").expect("Could not remove existing plugins dir");
    }
    
    // create new setup
    println!("Creating new setup...");
    fs::write(bin_path() + "/nocast.toml", "[plugins]").expect("Could not create config file");
    fs::create_dir(bin_path() + "/plugins/").expect("Could not create plugins dir");
}

fn main(){
    let args: Vec<String> = env::args().collect();
    if (args.len() < 2){
    	start_app();
	}
    else if args[1] == "setup" {
        setup();
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
