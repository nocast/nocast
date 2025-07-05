mod plugins;
mod config;
mod engine;
mod generic_types;
mod init;
mod monitor;
mod ui;
mod utils;
mod win;
mod app;
use crate::app::start_app;

fn main(){
    start_app();
}
