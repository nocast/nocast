use crate::utils::bin_path;
use std::process::Command;
use std::io::{self, Write};
use std::{fs, path::Path};
use toml::Value;
use crate::init::load_config;
use reqwest::blocking;
use std::error::Error;

fn build_crate(manifest_path: &str) -> Result<(), String> {
    // Execute the cargo command
    let status = Command::new("cargo")
        .arg("build")
        .arg("-q")
        .arg("--release")
        .arg("--manifest-path")
        .arg(manifest_path)
        .status()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    // Check exit status
    if status.success() {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

fn get_crate_name_from_dir(crate_dir: &str) -> Result<String, Box<dyn std::error::Error>> {
    let manifest_path = String::from(crate_dir) + "/Cargo.toml";
    let content = fs::read_to_string(manifest_path)?;
    let value: Value = toml::from_str(&content)?;
    
    value["package"]["name"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Package name missing or invalid".into())
}

fn run_git_clone(repo_url: &str, target_dir: Option<&str>) -> io::Result<()> {
    // Build the git command
    let mut cmd = Command::new("git");
    cmd.arg("clone").arg(repo_url);

    // Add target directory if provided
    if let Some(dir) = target_dir {
        cmd.arg(dir);
    }

    // Execute the command and check status
    let status = cmd.status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("git clone failed with exit code: {}", status),
        ))
    }
}

fn install_local_plugin(dir: String) {
    let target_path = bin_path() + "/plugins/";
    
	println!( "- Checking if it is a plugin...");
	// Validate the input is a directory
	if !(Path::new(&dir).is_dir()){
    	println!( "	Error: {} is not a valid directory", &dir);
    	std::process::exit(1);
	}
    let name: String = get_crate_name_from_dir(&dir).expect("Could not read plugin name");

	// Check for manifest.toml existence
	if (Path::new(&(dir.clone() + "/manifest.toml")).is_file()){
    	println!("	Success!")
	}
	else{
    	println!("	Error: manifest.toml not found in {}", &dir);
    	std::process::exit(1);
	}

	println!("- Building plugin `{}` from dir `{}`...", name, &dir);
    build_crate(&(dir.clone() + "/Cargo.toml"));

	// Install
	println!("- Installing plugin at `{}`...", &target_path);
    
    // copy manifest
	println!("	- Copying manifest...");
    let manifest_content: String = fs::read_to_string(dir.clone() + "/manifest.toml").expect("Could not read manifest");
    fs::write((target_path.clone() + &name) + ".toml", manifest_content).expect("Could not write new manifest");
    
    // copy core
	println!("	- Copying plugin core...");
    fs::copy(dir.clone() + "/target/release/lib" + &name + ".so", (target_path.clone() + &name) + ".so").expect("Could not copy core");

	// add to config
	println!("- Adding to config file...");
    let mut config_cont = load_config();
    config_cont.plugins.insert(name.clone(), format!("{}{}.toml", &target_path, &name));
	fs::write(bin_path() + "/nocast.toml", toml::to_string_pretty(&config_cont).expect("Could not generate config file")).expect("Could not write config file");
	
	println!("Plugin installed! :)");
}

fn fetch_ncpr(plugin: &str) -> Result<String, Box<dyn Error>> {
    let response = blocking::get("https://ncpr.roger-padrell.deno.net/api/repo/".to_string() + plugin)?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()).into());
    }

    let body = response.text()?;
    Ok(body)
}

fn install_git_plugin (repo: String) {
    // remove templugin for precaution
    if Path::new(&(bin_path() + "/templugin")).is_dir(){
        fs::remove_dir_all(bin_path() + "/templugin").expect("Could not remove templugin dir");
    }
    
    // Clone
    println!("- Cloning repo...");
	match run_git_clone(&repo, Some(&(bin_path() + "/templugin"))) {
        Ok(()) => println!("	Repository cloned successfully."),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        },
    }
    
    // Install
    install_local_plugin(bin_path() + "/templugin");
    
    // remove templguin
    fs::remove_dir_all(bin_path() + "/templugin").expect("Could not remove templugin dir");
}

pub fn install_plugin(name: String) {
	if name.contains(".git") && name.contains("http"){
        install_git_plugin(name);
    }
    else if Path::new(&name).is_dir(){
        install_local_plugin(name);
    }
    else{
        match fetch_ncpr(&name) {
        	Ok(body) => {
            	if body == "404"{
                	eprintln!("This plugin does not exist, it's not a git repo or a existing local directory.");
            	}
            	else{
                	install_git_plugin(body);
            	}
        	},
        	Err(e) => eprintln!("Error fetching Plugin Registry: {}", e),
    	}
    }
}

pub fn list_plugins() {
    let mut config_cont = load_config();
    for (name, plugin) in config_cont.plugins{
        println!("{}", name);
    }
}

pub fn uninstall_plugin(name: String) {
    // open config
    let mut config_cont = load_config();
    
	// check if plugin is installed
    if !config_cont.plugins.contains_key(&name){
        println!("Plugin '{}' is not installed or does not exist", &name);
        std::process::exit(1);
    }
    
    // remove plugin from config
    config_cont.plugins.remove(&name);
    fs::write(bin_path() + "/nocast.toml", toml::to_string_pretty(&config_cont).expect("Could not generate config file")).expect("Could not write config file");

	// remove core
    let target_path = bin_path() + "/plugins/";
    fs::remove_file((target_path.clone() + &name) + ".so").expect("Could not remove core");
    
    // remove manifest
    fs::remove_file((target_path.clone() + &name) + ".toml").expect("Could not remove manifest");
    
    println!("Successfully uninstalled '{}'!", &name);
}
