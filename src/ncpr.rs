use reqwest::blocking;
use std::error::Error;
use serde_json;

pub fn get_repo(plugin: &str) -> Result<String, Box<dyn Error>> {
    let response = blocking::get("https://ncpr.roger-padrell.deno.net/api/repo/".to_string() + plugin)?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()).into());
    }

    let body = response.text()?;
    Ok(body)
}

pub fn search_plugin(query: &str) -> Result<Vec<String>, Box<dyn Error>> {
	let response = blocking::get("https://ncpr.roger-padrell.deno.net/api/search/".to_string() + query)?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()).into());
    }

    let body: Vec<String> = response.json()?;
    Ok(body)
}
