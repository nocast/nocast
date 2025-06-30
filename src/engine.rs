use crate::generic_types::{Item, ActionOutput, NocastApp};
use regex::*;
use rustyscript::{json_args, Runtime, Module, Error};
use std::fs;
use std::str;

pub fn run_item(item: &Item) -> Vec<ActionOutput> {
    let file_bytes: &'static [u8] = include_bytes!("../plugincore.js");

    // Convert the byte slice to a string slice (assumes the file is valid UTF-8)
    let jsutils: &str = match str::from_utf8(file_bytes) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to parse file as UTF-8: {}", e);
            "console.error('Could not load plugincore.js')"
        }
    };

    // Load main JS script from file
    let script_code = fs::read_to_string(&item.path).expect("Could not read plugin content");

    // Combine utils and main script
    let full_script = format!("{}\n{}", jsutils, script_code);
    
    let module = Module::new("plugin.js", &full_script);

    let value: Vec<ActionOutput> = Runtime::execute_module(
        &module, vec![],
        Default::default(),
        json_args!(item.params)
    ).expect("Could not evaluate plugin");

    return value;
}

pub fn run_item_at(selecting: i16, main: &mut NocastApp){
    let new_item = &main.current_items[selecting as usize];
    let output = run_item(&new_item);
    let mut items: Vec<Item> = Vec::new();
    for o in output{
        let mut params: Vec<String> = Vec::new();
        params.push(main.query.clone());
        items.push(Item {title: (o.name), description: o.description, path: o.target, params, keyborad_shorcut: new_item.clone().keyborad_shorcut, image: new_item.clone().image});
    }
    main.current_items = items;
}

pub fn query_items(main: &NocastApp) -> Vec<Item> {
    let mut items: Vec<Item> = Vec::new();
    for pl in main.plugins.clone(){
        for a in pl.actions{
            let re = Regex::new(&a.expression).unwrap();
            if let Some(caps) = re.captures(&main.query) {
                // Skip group 0 (the full match), collect only capture groups
                let groups: Vec<String> = caps
                    .iter()
                    .map(|m| m.map_or("".to_string(), |mat| mat.as_str().to_string()))
                    .collect();

                let new_item = Item { title: (&a.name).clone(), description: ((&pl.name).clone()), image: (String::from("")), keyborad_shorcut: (String::from("")), path: ((&pl.path).clone() + a.file.as_str()), params: (groups) };

                if a.autorun{
                    let output = run_item(&new_item);
                    for o in output{
                        let mut params: Vec<String> = Vec::new();
                        params.push(main.query.clone());
                        items.push(Item {title: (o.name), description: o.description, path: o.target, params, keyborad_shorcut: new_item.clone().keyborad_shorcut, image: new_item.clone().image});
                    }
                }
                else{
                    items.push(new_item);
                }
            }  
        }
    }
    return items;
}