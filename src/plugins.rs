use crate::generic_types;
use std::fs;
use crate::utils::bin_path;

pub fn test_plugins(config: &generic_types::Config) -> Vec<generic_types::PluginManifest>{
    let mut res: Vec<generic_types::PluginManifest> = Vec::new();
    for p in config.plugins.clone(){
        let manifest_path = bin_path() + (p.1).as_str();
        let toml_content = fs::read_to_string(manifest_path).expect(&format!("Could not read manifest from plugin '{}'", p.0));

        let cont: generic_types::PluginManifest = toml::from_str(&toml_content).expect(&format!("Could not parse plugin '{}'", p.0));
        res.push(cont);
    }
    return res;
}