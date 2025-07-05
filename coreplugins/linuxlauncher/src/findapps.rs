use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[derive(Debug)]
pub struct AppInfo {
    pub name: String,
    pub exec_path: String,
    pub comment: Option<String>,
    pub generic_name: Option<String>,
}

/// Detects installed applications by parsing .desktop files
pub fn detect_applications() -> Vec<AppInfo> {
    let mut applications = Vec::new();
    let desktop_dirs = get_desktop_file_dirs();

    for dir in desktop_dirs {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("desktop") {
                    if let Some(app) = parse_desktop_file(&path) {
                        applications.push(app);
                    }
                }
            }
        }
    }

    applications
}

/// Returns standard directories for .desktop files
fn get_desktop_file_dirs() -> Vec<PathBuf> {
    let home_dir = dirs::home_dir().expect("Home directory not found");
    let mut dirs = vec![
        PathBuf::from("/usr/share/applications"),
        PathBuf::from("/usr/local/share/applications"),
        home_dir.join(".local/share/applications"),
    ];

    if let Some(xdg_dirs) = std::env::var_os("XDG_DATA_DIRS") {
        for dir in std::env::split_paths(&xdg_dirs) {
            let app_dir = dir.join("applications");
            if app_dir.is_dir() {
                dirs.push(app_dir);
            }
        }
    }

    dirs
}

/// Parses a .desktop file to extract app information
fn parse_desktop_file(path: &Path) -> Option<AppInfo> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return None,
    };
    
    let mut properties = HashMap::new();
    let mut in_desktop_entry = false;

    for line in content.lines() {
        let line = line.trim();
        
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            in_desktop_entry = line == "[Desktop Entry]";
            continue;
        }

        if in_desktop_entry {
            if let Some((key, value)) = line.split_once('=') {
                properties.insert(key.trim(), value.trim());
            }
        }
    }

    if properties.get("Type")? != &"Application" {
        return None;
    }

    if properties.get("NoDisplay") == Some(&"true") 
        || properties.get("Hidden") == Some(&"true") {
        return None;
    }

    let name = properties.get("Name")?.to_string();
    
    // Extract optional fields
    let comment = properties.get("Comment").map(|s| s.to_string());
    let generic_name = properties.get("GenericName").map(|s| s.to_string());

    // Clean executable path
    let exec = properties.get("Exec")?;
    let clean_exec = clean_exec_string(exec);

    Some(AppInfo {
        name,
        exec_path: clean_exec,
        comment,
        generic_name,
    })
}

/// Cleans executable string from .desktop files
fn clean_exec_string(exec: &str) -> String {
    // Split into components and take the first non-parameter token
    let first_token = exec.split_whitespace()
        .find(|token| !token.starts_with('%') && !token.starts_with('@'))
        .unwrap_or(exec);

    // Remove surrounding quotes
    first_token.trim_matches(|c| c == '"' || c == '\'').to_string()
}
