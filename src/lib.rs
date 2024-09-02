mod window;

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

use window::{Window, WindowEvents};

pub fn run() {
    env_logger::init();

    let window = Window::new("my first voxel engine");

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
