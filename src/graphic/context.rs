/* Glium context */
#![allow(dead_code)]

// Crates using
use crate::resource::settings::Settings;
use glium::{
    Display,
    glutin::{
        ContextBuilder,
        dpi::LogicalSize,
        event_loop::EventLoop,
        window::WindowBuilder
    }
};

// Structures
#[derive(Debug)]
pub struct Context {
    pub event_loop: EventLoop<()>,
    pub display: Display
}

// Implementations
impl Context {
    // Create context according to settings
    pub fn new(s: &Settings) -> Result<Context, String> {
        // Create event loop
        let event_loop = EventLoop::new();
 
        // Create window builder with settings
        let wb = WindowBuilder::new()
            .with_inner_size(
                LogicalSize::new(
                    s.graphics.resolution[0], 
                    s.graphics.resolution[1]
                )
            )
            .with_title("Open Craft");
       
        // Create context buider
        let cb = ContextBuilder::new();

        // Create display
        let display = Display::new(wb, cb, &event_loop);
        match display {
            // Success
            Ok(d)   => Ok(Context {
                event_loop,
                display: d
            }),

            // Fail
            // Maybe error on creating backend or OpenGL impementation too old
            Err(_)  => Err("Fail to create display".to_string())
        }
    }
}

