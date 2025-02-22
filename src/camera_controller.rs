use cgmath::{InnerSpace, Vector3};
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::camera::Camera;

pub struct CameraController {
    pub speed: f32,
    pub sensitivity: f32, // Mouse sensitivity
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
    pub is_up_pressed: bool,
    pub is_down_pressed: bool,
    pub mouse_delta: (f32, f32), // (delta_x, delta_y)
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            speed,
            sensitivity,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_up_pressed: false,
            is_down_pressed: false,
            mouse_delta: (0.0, 0.0),
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        physical_key: PhysicalKey::Code(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    KeyCode::KeyW | KeyCode::ArrowUp => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyA | KeyCode::ArrowLeft => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyS | KeyCode::ArrowDown => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyD | KeyCode::ArrowRight => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    KeyCode::Space => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    KeyCode::ShiftLeft => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn process_mouse(&mut self, delta_x: f32, delta_y: f32) {
        self.mouse_delta = (delta_x, delta_y);
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        // Update pitch and yaw based on mouse movement
        camera.yaw += self.mouse_delta.0 * self.sensitivity;
        camera.pitch -= self.mouse_delta.1 * self.sensitivity;
        camera.pitch = camera.pitch.clamp(-89.0, 89.0);

        // Reset mouse delta after processing
        self.mouse_delta = (0.0, 0.0);

        // Update the camera's target
        camera.update_target();

        // Calculate movement direction
        let forward = (camera.target - camera.eye).normalize();
        let right = forward.cross(camera.up).normalize();

        // Update position based on input
        if self.is_forward_pressed {
            camera.eye += forward * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward * self.speed;
        }
        if self.is_right_pressed {
            camera.eye += right * self.speed;
        }
        if self.is_left_pressed {
            camera.eye -= right * self.speed;
        }

        // Move up and down
        if self.is_up_pressed {
            camera.eye += camera.up * self.speed; // Move up
        }
        if self.is_down_pressed {
            camera.eye -= camera.up * self.speed; // Move down
        }

        // Update the target after moving the eye
        camera.target = camera.eye + forward;
    }

    pub fn re_center_mouse(&self, window: &Window) {
        let window_size = window.inner_size();
        let center_x = window_size.width as f64 / 2.0;
        let center_y = window_size.height as f64 / 2.0;

        if let Err(e) = window.set_cursor_position(PhysicalPosition::new(center_x, center_y)) {
            eprintln!("Failed to re-center cursor: {:?}", e);
        }
    }
}
