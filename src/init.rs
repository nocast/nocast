use crate::generic_types;
use crate::plugins::test_plugins;
use crate::utils;
use std::fs;

pub fn load_config() -> generic_types::Config {
    let path = utils::bin_path();

    let toml_content = fs::read_to_string(path + "/nocast.toml").expect("Could not read configuration file");

    let cont: generic_types::Config = toml::from_str(&toml_content).expect("Could not parse configuration file");

    return cont;
}

pub fn load_plugins(config: &generic_types::Config) -> Vec<generic_types::Plugin> {
    // test and read Vec<PluginManifest>
    let manifests = test_plugins(&config);
    let path = utils::bin_path();

    // transform Vec<PluginManifest> into Vec<Plugin>
    let mut plugins: Vec<generic_types::Plugin> = Vec::new();
    for m in manifests{
        plugins.push(generic_types::Plugin{
            name: m.name.clone(),
            path: path.clone() + config.plugins.get(&m.name).expect("Could not read plugin path").as_str(),
            actions: m.actions, 
        })
    }

    return plugins;
}