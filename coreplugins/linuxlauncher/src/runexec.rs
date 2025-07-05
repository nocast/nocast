pub fn run_exec(path: &str){
    std::process::Command::new("sh")
    	.arg("-c")
   	 .arg(format!("nohup {} >/dev/null 2>&1 &", path))
   	 .spawn().expect("Error spawning process");
}
