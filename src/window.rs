use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window,
};

pub enum WindowEvents {
    Resized {
        width: u32,
        height: u32,
    },
    Keyboard {
        state: ElementState,
        virtual_keycode: PhysicalKey,
    },
    Draw,
}

pub struct Window {
    pub event_loop: EventLoop<()>,
    pub window: window::Window,
}

impl Window {
    pub fn new(title: &str) -> Self {
        let event_loop = EventLoop::new().unwrap();

        let window = window::WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();

        Self { event_loop, window }
    }

    // run event_loop
    pub fn run(self, mut callback: impl 'static + FnMut(WindowEvents) -> ()) {
        self.event_loop
            .run(move |event, control_flow| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => control_flow.exit(),
                    WindowEvent::Resized(physical_size) => callback(WindowEvents::Resized {
                        width: physical_size.width,
                        height: physical_size.height,
                    }),
                    WindowEvent::KeyboardInput {
                        device_id,
                        event,
                        is_synthetic,
                    } => callback(WindowEvents::Keyboard {
                        state: event.state,
                        virtual_keycode: event.physical_key,
                    }),
                    _ => {}
                },
                _ => {}
            })
            .unwrap();
    }
}
