use std::error::Error;

use tetra::graphics::Camera;
use tetra::input::Key;
use tetra::Context;

use crate::graph::Position;

const CAMERA_ZOOM_SPEED: f32 = 0.05;
pub const Y_AXIS_MOVE_SPEED: f32 = 10.;
pub const X_AXIS_MOVE_SPEED: f32 = 10.;
const ROTATION_SPEED: f32 = 0.05;

pub trait CameraState {
    fn handle_camera_events(&mut self, event: tetra::Event) -> Result<(), Box<dyn Error>>;

    fn update_camera_transformation(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>>;
}

impl CameraState for Camera {
    fn handle_camera_events(&mut self, event: tetra::Event) -> Result<(), Box<dyn Error>> {
        // Only y coordinate is accessed because x corresponds to horizontal move of mouse wheel.
        if let tetra::Event::MouseWheelMoved { amount } = &event {
            if amount.y > 0 {
                self.scale += CAMERA_ZOOM_SPEED;
            } else {
                self.scale -= CAMERA_ZOOM_SPEED;
            }
        }

        if let tetra::Event::KeyPressed { key: Key::W } = &event {
            self.position += Position::unit_y().rotated_z(-self.rotation) * Y_AXIS_MOVE_SPEED;
        }

        if let tetra::Event::KeyPressed { key: Key::S } = &event {
            self.position -= Position::unit_y().rotated_z(-self.rotation) * Y_AXIS_MOVE_SPEED;
        }

        if let tetra::Event::KeyPressed { key: Key::A } = &event {
            self.position += Position::unit_x().rotated_z(-self.rotation) * X_AXIS_MOVE_SPEED;
        }

        if let tetra::Event::KeyPressed { key: Key::D } = &event {
            self.position -= Position::unit_x().rotated_z(-self.rotation) * X_AXIS_MOVE_SPEED;
        }

        self.update();

        Ok(())
    }

    fn update_camera_transformation(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
        if tetra::input::is_key_down(ctx, Key::Q) {
            self.rotation += ROTATION_SPEED;
        }
        if tetra::input::is_key_down(ctx, Key::E) {
            self.rotation -= ROTATION_SPEED;
        }

        self.update();

        Ok(())
    }
}
