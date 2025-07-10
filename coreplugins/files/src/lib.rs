use nocast_plugincore::*;
mod search;
use crate::search::fast_search;
use std::env;

#[unsafe(no_mangle)]
pub extern "C" fn sample(input: plugin_input) -> plugin_output {
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
        res.push(ActionOutput {name: path.clone(), description: "Copy path to clipboard".to_string(), target: ("copy,".to_owned() + &path).to_string()});
    }

    return prepare_output(res);
}
