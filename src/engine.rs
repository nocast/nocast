use crate::generic_types::{Item, ActionOutput, NocastApp};
use regex::*;
use std::fs;
use std::str;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use libloading::{Library, Symbol};

type PluginFunction = unsafe fn(input: *const c_char) -> *mut c_char;

pub fn run_plugin(path: String, target_function: String, input: Vec<String>) -> Vec<ActionOutput> {
    let input_json = serde_json::to_string(&input).unwrap();
    let c_input = CString::new(input_json).unwrap();

    unsafe {
        let lib = Library::new(path).unwrap(); // Change path accordingly
        let func: Symbol<PluginFunction> = lib.get(target_function.as_bytes()).unwrap();

        let raw_output = func(c_input.as_ptr());
        let output_cstr = CString::from_raw(raw_output);
        let output_str = output_cstr.to_str().unwrap();

        let outputs: Vec<ActionOutput> = serde_json::from_str(output_str).unwrap();
        return outputs;
    }
}

pub fn run_item(item: &Item) -> Vec<ActionOutput> {
    // Add item running
    return run_plugin(item.clone().path, item.clone().function, item.clone().params);
}

pub fn run_item_at(selecting: i16, main: &mut NocastApp){
    let new_item = &main.current_items[selecting as usize];
    let output = run_item(&new_item);
    let mut items: Vec<Item> = Vec::new();
    for o in output{
        let mut params: Vec<String> = Vec::new();
        params.push(main.query.clone());
        items.push(Item {title: (o.name), description: o.description, path: new_item.clone().path, function: o.target, params, keyborad_shorcut: new_item.clone().keyborad_shorcut, image: new_item.clone().image});
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

                let new_item = Item { title: (&a.name).clone(), description: ((&pl.name).clone()), image: (String::from("")), keyborad_shorcut: (String::from("")), path: ((&pl.path).clone()), params: (groups), function:  a.function };

                if a.autorun{
                    let output = run_item(&new_item);
                    for o in output{
                        let mut params: Vec<String> = Vec::new();
                        params.push(main.query.clone());
                        items.push(Item {title: (o.name), description: o.description, path: new_item.clone().path, function: o.target,  params, keyborad_shorcut: new_item.clone().keyborad_shorcut, image: new_item.clone().image});
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