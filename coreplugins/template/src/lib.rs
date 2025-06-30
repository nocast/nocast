use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionOutput {
    pub name: String,
    pub description: String,
    pub target: String,
}

#[unsafe(no_mangle)]
pub extern "C" fn process_input(input: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(input) };
    let input_str = c_str.to_str().unwrap();
    let input_vec: Vec<String> = serde_json::from_str(input_str).unwrap();

    let mut res: Vec<ActionOutput> = Vec::new();
    res.push(ActionOutput {name: "some".to_string(), description: "thing".to_string(), target: "else".to_string()});

    let output_json = serde_json::to_string(&res).unwrap();
    CString::new(output_json).unwrap().into_raw()
}

