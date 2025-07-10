use nocast_plugincore::*;
mod findapps;
mod runexec;

#[unsafe(no_mangle)]
pub extern "C" fn searchapp(input: plugin_input) -> plugin_output {
    let input_vec: Vec<String> = parse_input(input);
	let apps: Vec<findapps::AppInfo> = findapps::detect_applications();
    let query = &input_vec[1];
    
    let mut res: Vec<ActionOutput> = Vec::new();
    for a in apps{
        if a.name.to_lowercase().contains(&query.to_lowercase()){
            res.push(ActionOutput {name: a.name, description: a.comment.unwrap_or(String::new()), target: "openapp,".to_string()+&a.exec_path});
        }    
    }

    return prepare_output(res);
}

#[unsafe(no_mangle)]
pub extern "C" fn openapp(input: plugin_input) -> plugin_output {
    let input_vec: Vec<String> = parse_input(input);
    
    runexec::run_exec(&input_vec[0]);
    
    let mut res: Vec<ActionOutput> = Vec::new();
	res.push(ActionOutput {name: "{exit}".to_string(), description: String::new(), target: "exit,now".to_string()});
    return prepare_output(res);
}
