mod window;
mod graphics;

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::{
    window::{ Window, WindowEvents },
    graphics::Graphics,
};

pub async fn run() {
    env_logger::init();

    let mut window = Window::new("my first voxel engine");

    let graphics = Graphics::new(&window).await;

    window.run(move |event| match event {
        WindowEvents::Resized { width, height } => {
            // state.resize(winit::dpi::PhysicalSize { width, height });
            // todo!();
        }
        WindowEvents::Draw => {
            //state.update();
            //state.render();
        }
        WindowEvents::Keyboard {
            state,
            virtual_keycode,
        } => {
            println!("{:?}", state);
            println!("{:?}", virtual_keycode);
        }
    });
}
