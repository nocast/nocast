use winit::{dpi::PhysicalPosition, monitor::MonitorHandle};
use crate::monitor::{get_active_monitor, get_monitor_center};
use egui::{Pos2, Vec2, Color32};

const WIDTH: f32 = 0.38;
const HEIGHT: f32 = 0.05;
pub const MAX_ITEMS: f32 = 5.0;

// Convert winit physical position to egui Pos2
pub fn physical_to_pos2(physical: PhysicalPosition<f32>) -> Pos2 {
    Pos2::new(physical.x, physical.y)
}

pub fn get_relative_window_pos(m: &MonitorHandle) -> Pos2{
    let monitor = m.clone();
    let monitor_center = get_monitor_center(&monitor);
    let offset_x = (monitor.size().width as f32) * WIDTH / 2.0;
    let offset_y = (monitor.size().height as f32) * HEIGHT / 2.0;
    let offseted_center = PhysicalPosition {x: ((monitor_center.x as f32) - offset_x), y: ((monitor_center.y as f32) - offset_y)};
    return physical_to_pos2(offseted_center);
}

pub fn get_window_size(m: &MonitorHandle) -> Vec2{
    let monitor = m.clone();
    let offset_x = (monitor.size().width as f32) * WIDTH;
    let offset_y = (monitor.size().height as f32) * HEIGHT;
    return Vec2::new(offset_x as f32, offset_y as f32)
}

pub fn get_window() -> (Pos2, Vec2){
    let monitor = get_active_monitor();
    return (get_relative_window_pos(&monitor), get_window_size(&monitor));
}