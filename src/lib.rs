mod graphics;

use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::graphics::Graphics;

pub async fn run() {
    env_logger::init();

    // window.run(move |event| match event {
    //     WindowEvents::Resized { width, height } => {
    //         // state.resize(winit::dpi::PhysicalSize { width, height });
    //         // todo!();
    //     }
    //     WindowEvents::Draw => {
    //         //state.update();
    //         //state.render();
    //     }
    //     WindowEvents::Keyboard {
    //         state,
    //         virtual_keycode,
    //     } => {
    //         println!("{:?}", state);
    //         println!("{:?}", virtual_keycode);
    //     }
    // });

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
            } if window_id == graphics.window().id() => if !graphics.input(event) {
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

                WindowEvent::KeyboardInput {
                    device_id,
                    event,
                    is_synthetic,
                } => {
                    println!("{:?}", event);
                }
                _ => {}
               }
            },
            _ => {}
        }
        )
        .unwrap();
}
