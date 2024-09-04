mod graphics;

use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::graphics::Graphics;

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_title("my_first_voxel_engine")
        .build(&event_loop)
        .unwrap();

    let mut graphics = Graphics::new(&window).await;

    event_loop
        .run(move |event, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == graphics.window().id() => {
                if !graphics.input(event) {
                    match event {
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
                        WindowEvent::Resized(physical_size) => graphics.resize(*physical_size),
                        WindowEvent::RedrawRequested => {
                            graphics.update();
                            match graphics.render() {
                                Ok(_) => {}
                                // Reconfigure the surface if lost
                                Err(wgpu::SurfaceError::Lost) => graphics.resize(graphics.size),
                                // The system is out of memory, we should probably quit
                                Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),
                                // All other errors (Outdated, Timeout) should be resolved by the next frame
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }

                        WindowEvent::KeyboardInput {
                            device_id,
                            event,
                            is_synthetic,
                        } => {
                            println!("{:?}", event);
                        }
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                // RedrawRequested will only trigger once unless we manually
                // request it.
                graphics.window().request_redraw();
            }
            _ => {}
        })
        .unwrap();
}
