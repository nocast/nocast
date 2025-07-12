use std::env;
use std::path::PathBuf;

pub fn bin_path() -> String{
    match env::current_exe() {
        Ok(exe_path) => {
            let dir = exe_path.parent().unwrap();
            return String::from(dir.to_str().expect("Could not get bin path"));
        }
        Err(e) => {
            return String::from(".");
        }
    }
}