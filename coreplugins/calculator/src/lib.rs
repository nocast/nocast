use nocast_plugincore::*;
use meval::eval_str;
use arboard::Clipboard;
use std::collections::HashMap;

#[unsafe(no_mangle)]
pub extern "C" fn operation(input: plugin_input) -> plugin_output {
    let input_vec: Vec<String> = parse_input(input);

	match eval_str(&input_vec[1]) {
        Ok(result) => {
            let mut res: Vec<ActionOutput> = Vec::new();
    		res.push(ActionOutput {name: (&result).to_string(), description: input_vec[1].clone(), target: format!("copy_result,{}", &result)});
    		return prepare_output(res);
    	},
        Err(_) => {
			 return prepare_output(Vec::new());
        },
    }
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

#[unsafe(no_mangle)]
pub extern "C" fn lenght_convert(input: plugin_input) -> plugin_output {
    let input_vec: Vec<String> = parse_input(input);
    
    // Metric conversions in meters
    let mut unit_to_meter: HashMap<&str, f64> = HashMap::new();
    unit_to_meter.insert("mm", 0.001);
    unit_to_meter.insert("millimeter", 0.001);
    unit_to_meter.insert("millimeters", 0.001);
    unit_to_meter.insert("cm", 0.01);
    unit_to_meter.insert("centimeter", 0.01);
    unit_to_meter.insert("centimeters", 0.01);
    unit_to_meter.insert("m", 1.0);
    unit_to_meter.insert("meter", 1.0);
    unit_to_meter.insert("meters", 1.0);
    unit_to_meter.insert("km", 1000.0);
    unit_to_meter.insert("kilometer", 1000.0);
    unit_to_meter.insert("kilometers", 1000.0);

    // Imperial
    unit_to_meter.insert("in", 0.0254);
    unit_to_meter.insert("inch", 0.0254);
    unit_to_meter.insert("inches", 0.0254);
    unit_to_meter.insert("ft", 0.3048);
    unit_to_meter.insert("foot", 0.3048);
    unit_to_meter.insert("feet", 0.3048);
    unit_to_meter.insert("yd", 0.9144);
    unit_to_meter.insert("yard", 0.9144);
    unit_to_meter.insert("yards", 0.9144);
    unit_to_meter.insert("mi", 1609.344);
    unit_to_meter.insert("mile", 1609.344);
    unit_to_meter.insert("miles", 1609.344);

    let original_val: f64 = (&input_vec[2]).parse().unwrap();
    let original_unit = &input_vec[5];
    let target_unit = &input_vec[7];
    
    let target_val = original_val * unit_to_meter.get(original_unit.as_str()).unwrap() / unit_to_meter.get(target_unit.as_str()).unwrap();

	let mut res: Vec<ActionOutput> = Vec::new();
	res.push(ActionOutput {name: (&target_val).to_string(), description: input_vec[1].clone(), target: format!("copy_result,{}", &target_val)});
    return prepare_output(res);
}

