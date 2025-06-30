use winit::dpi::PhysicalPosition;
use winit::monitor::{MonitorHandle};
use winit::event_loop::EventLoop;
use device_query::{DeviceQuery, DeviceState};

pub fn get_all_monitors() -> Vec<MonitorHandle>{
    // Create an event loop (required for monitor access)
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    // Retrieve all connected monitors
    let monitors: Vec<MonitorHandle> = event_loop.available_monitors().collect();
    return monitors;
}

pub fn get_ordered_monitors() -> Vec<MonitorHandle>{
    // Create an event loop (required for monitor access)
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    // Retrieve all connected monitors
    let mut monitors: Vec<MonitorHandle> = event_loop.available_monitors().collect();
    monitors.sort_by_key(|monitor| monitor.position().x);
    return monitors;
}

pub fn get_primary_monitor() -> MonitorHandle{
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let default_monitor = event_loop.primary_monitor()
        .or_else(|| event_loop.available_monitors().next())
        .expect("No monitors available");
    return default_monitor;
}

pub fn get_active_monitor() -> MonitorHandle{
    let device_state = DeviceState::new();
    let (mouse_x, mouse_y) = device_state.get_mouse().coords;
    let monitors = get_ordered_monitors();

    let current_monitor = monitors.iter().find(|monitor| {
        let pos = monitor.position();
        let size = monitor.size();
        
        // Check if mouse is within monitor bounds
        mouse_x >= pos.x as i32 &&
        mouse_x < (pos.x + size.width as i32) as i32 &&
        mouse_y >= pos.y as i32 &&
        mouse_y < (pos.y + size.height as i32) as i32
    });

    match current_monitor{
        Some(monitor) => {
            return monitor.clone();
        }
        None =>{
            return get_primary_monitor();
        }
    }
}

pub fn get_monitor_center(m: &MonitorHandle) -> PhysicalPosition<u32>{
    let mon = m.clone();
    let position = mon.position();
    let size = mon.size();
    let center_x = (position.x as u32) + (size.width/2);
    let center_y = (position.y as u32) + (size.height/2);
    return PhysicalPosition { x: (center_x), y: (center_y) }
}