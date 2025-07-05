use nocast_plugincore::*;

#[unsafe(no_mangle)]
pub extern "C" fn sample(input: plugin_input) -> plugin_output {
    let input_vec: Vec<String> = parse_input(input);

    let mut res: Vec<ActionOutput> = Vec::new();
    res.push(ActionOutput {name: "some".to_string(), description: "thing".to_string(), target: "else,a".to_string()});

    return prepare_output(res);
}
