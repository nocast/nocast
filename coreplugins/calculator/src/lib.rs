use nocast_plugincore::*;
use meval::eval_str;
use arboard::Clipboard;

#[unsafe(no_mangle)]
pub extern "C" fn operation(input: plugin_input) -> plugin_output {
    let input_vec: Vec<String> = parse_input(input);
    
    let result: f64 = eval_str(&input_vec[1]).expect("Could not solve math operation");
    
    let mut res: Vec<ActionOutput> = Vec::new();
    
    res.push(ActionOutput {name: (&result).to_string(), description: input_vec[1].clone(), target: format!("copy_result,{}", &result)});

    return prepare_output(res);
}

#[unsafe(no_mangle)]
pub extern "C" fn copy_result(input: plugin_input) -> plugin_output {
	let input_vec: Vec<String> = parse_input(input);
    
    let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
    clipboard.set_text(input_vec[0].clone()).expect("Failed to set clipboard contents");
    
    let mut res: Vec<ActionOutput> = Vec::new();
	res.push(ActionOutput {name: "{exit}".to_string(), description: String::new(), target: "exit,now".to_string()});
    return prepare_output(res);
}
