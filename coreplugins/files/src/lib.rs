use nocast_plugincore::*;
mod search;
use crate::search::fast_search;
use std::env;
use arboard::Clipboard;

#[unsafe(no_mangle)]
pub extern "C" fn search_files(input: plugin_input) -> plugin_output {
    let input_vec: Vec<String> = parse_input(input);
    
    if &input_vec[1] == ""{
    	return prepare_output(Vec::new());
    }
    
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE")) // Windows fallback
        .expect("Could not determine the home directory");
    let results = fast_search(&home_dir, &input_vec[1]);
    
    let mut res: Vec<ActionOutput> = Vec::new();
    for path in results {
        res.push(ActionOutput {name: path.clone(), description: "Copy path to clipboard".to_string(), target: ("copy_path,".to_owned() + &path).to_string()});
    }

    return prepare_output(res);
}

#[unsafe(no_mangle)]
pub extern "C" fn copy_path(input: plugin_input) -> plugin_output {
	let input_vec: Vec<String> = parse_input(input);
    
    let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
    clipboard.set_text(input_vec[0].clone()).expect("Failed to set clipboard contents");
    
    let mut res: Vec<ActionOutput> = Vec::new();
	res.push(ActionOutput {name: "{exit}".to_string(), description: String::new(), target: "exit,now".to_string()});
    return prepare_output(res);
}
