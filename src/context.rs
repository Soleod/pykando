use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct Context {
    pub event_loop: EventLoop<()>,
    pub window: Window,
}

impl Context {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .build(&event_loop)
            .expect("Failed to create window");
        Context {
            event_loop,
            window,
        }
    }
}