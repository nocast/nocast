use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use dark_light::Mode;

pub type PluginConfig = (String, HashMap<String, bool>); // (Path, <ActionName, active?>)

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum ThemeMode {
	Light, Dark, Unspecified
}

impl ThemeMode {
    pub fn all() -> [ThemeMode; 3] {
        [ThemeMode::Dark, ThemeMode::Light, ThemeMode::Unspecified]
    }

    pub fn label(&self) -> &'static str {
        match self {
            ThemeMode::Dark => "Dark",
            ThemeMode::Light => "Light",
            ThemeMode::Unspecified => "System",
        }
    }
}

pub fn parse(mode: Mode) -> ThemeMode {
    match mode {
        Mode::Dark => ThemeMode::Dark,
        Mode::Light => ThemeMode::Light,
        Mode::Unspecified => ThemeMode::Unspecified,
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub plugins: HashMap<String, PluginConfig>,
	pub theme: ThemeMode
}

#[derive(Debug, Deserialize, Clone)]
pub struct PluginAction{
    pub name: String, 
    pub expression: String,
    pub function: String,
    pub autorun: bool
}   

#[derive(Debug, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub author: String,
    pub license: String,
    pub source: String,
    pub actions: Vec<PluginAction>
}

#[derive(Clone)]
pub struct Plugin {
    pub name: String,
    pub path: String, 
    pub actions: Vec<PluginAction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionOutput {
    pub name: String,
    pub description: String,
    pub target: String,
}

#[derive(Clone)]
pub struct Item {
    pub title: String,
    pub description: String,
    pub image: String, 
    pub keyborad_shorcut: String,
    pub path: String,
    pub params: Vec<String>,
    pub function: String,
    pub content: String
}

pub struct NocastApp {
    pub query: String,
    pub wc: (egui::Pos2, egui::Vec2),
    pub selecting: i16,
    pub previous_query: String,
    pub plugins: Vec<Plugin>,
    pub config: Config,
    pub current_items: Vec<Item>,
    pub theme: crate::config::Theme
}