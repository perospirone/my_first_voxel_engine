use instant::Instant;
use winit::{
    event::{DeviceEvent, ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::CursorGrabMode,
};
mod camera;
mod camera_controller;
mod graphics;
mod texture;

use crate::graphics::Graphics;

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_title("my_first_voxel_engine")
        .build(&event_loop)
        .unwrap();

    let mut graphics = Graphics::new(&window).await;

    // Grab the cursor and hide it
    if let Err(e) = window.set_cursor_grab(CursorGrabMode::Confined) {
        eprintln!("Failed to grab cursor: {:?}", e);
    }
    graphics.window.set_cursor_visible(false);

    let mut last_frame_time = instant::Instant::now();

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
                        WindowEvent::Focused(focused) => {
                            // Re-grab the cursor and hide it when the window is focused
                            if *focused {
                                if let Err(e) =
                                    graphics.window.set_cursor_grab(CursorGrabMode::Confined)
                                {
                                    eprintln!("Failed to grab cursor: {:?}", e);
                                }
                                graphics.window.set_cursor_visible(false);
                            }
                        }
                        WindowEvent::RedrawRequested => {
                            let now = instant::Instant::now();
                            let dt = now.duration_since(last_frame_time).as_secs_f32();
                            last_frame_time = now;

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
                        _ => {}
                    }
                }
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                graphics
                    .camera_controller
                    .process_mouse(delta.0 as f32, delta.1 as f32);
            }
            Event::AboutToWait => {
                graphics.update();
                graphics.window().request_redraw();
            }
            _ => {}
        })
        .unwrap();
}
