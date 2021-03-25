use winit::window::Window;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent::*, WindowEvent, ElementState};

fn process_event(window: &Window, event: Event<()>, control_flow: &mut ControlFlow) {
    match event {
        Event::WindowEvent { event, window_id, .. } => match event {
            Resized(_physical_device) => {}
            Moved(_physical_position) => {}
            CloseRequested => {
                if window_id == window.id() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Destroyed => {}
            DroppedFile(_) => {}
            HoveredFile(_) => {}
            HoveredFileCancelled => {}
            ReceivedCharacter(_) => {}
            Focused(_) => {}
            KeyboardInput { device_id, input, is_synthetic } => {
                if input.state == ElementState::Released {
                    println!("{}", input.scancode);
                }
            }
            ModifiersChanged(_) => {}
            CursorMoved { .. } => {}
            CursorEntered { .. } => {}
            CursorLeft { .. } => {}
            MouseWheel { .. } => {}
            MouseInput { .. } => {}
            TouchpadPressure { .. } => {}
            AxisMotion { .. } => {}
            Touch(_) => {}
            ScaleFactorChanged { .. } => {}
            ThemeChanged(_) => {}
        },
        Event::NewEvents(_) => {}
        Event::DeviceEvent { .. } => {}
        Event::UserEvent(_) => {}
        Event::Suspended => {}
        Event::Resumed => {}
        Event::MainEventsCleared => {}
        Event::RedrawRequested(_) => {}
        Event::RedrawEventsCleared => {}
        Event::LoopDestroyed => {}
    }
}


pub fn run(window: Window, event_loop: EventLoop<()>) {
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        process_event(&window, event, control_flow);
    })
}
