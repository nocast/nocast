use crate::utils::bin_path;
use std::fs;

pub fn setup(){
    // Config and plugin dirs
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
    fs::write(bin_path() + "/nocast.toml", "theme = \"Unspecified\"\n[plugins]").expect("Could not create config file");
    fs::create_dir(bin_path() + "/plugins/").expect("Could not create plugins dir");
}
