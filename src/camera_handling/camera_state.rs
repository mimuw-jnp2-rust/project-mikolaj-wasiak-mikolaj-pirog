use std::error::Error;

use tetra::graphics::Camera;
use tetra::input::Key;
use tetra::Context;

use crate::constants::{CAMERA_ZOOM_SPEED, ROTATION_SPEED, X_AXIS_MOVE_SPEED, Y_AXIS_MOVE_SPEED};
use crate::graph::Position;

pub trait CameraState {
    fn handle_camera_events(&mut self, event: tetra::Event) -> Result<(), Box<dyn Error>>;

    fn update_camera_transformation(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>>;
}

impl CameraState for Camera {
    fn handle_camera_events(&mut self, event: tetra::Event) -> Result<(), Box<dyn Error>> {
        // Only y coordinate is accessed because x corresponds to horizontal move of mouse wheel.
        if let tetra::Event::MouseWheelMoved { amount } = &event {
            if amount.y > 0 {
                if !(self.scale.x >= f32::MAX - CAMERA_ZOOM_SPEED
                    || self.scale.y >= f32::MAX - CAMERA_ZOOM_SPEED)
                {
                    self.scale += CAMERA_ZOOM_SPEED;
                }
            } else if (self.scale - CAMERA_ZOOM_SPEED).are_all_positive() {
                self.scale -= CAMERA_ZOOM_SPEED;
            }
        }

        self.update();

        Ok(())
    }

    fn update_camera_transformation(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
        if tetra::input::is_key_down(ctx, Key::W) {
            self.position += Position::unit_y().rotated_z(-self.rotation) * Y_AXIS_MOVE_SPEED;
        }

        if tetra::input::is_key_down(ctx, Key::S) {
            self.position -= Position::unit_y().rotated_z(-self.rotation) * Y_AXIS_MOVE_SPEED;
        }

        if tetra::input::is_key_down(ctx, Key::A) {
            self.position += Position::unit_x().rotated_z(-self.rotation) * X_AXIS_MOVE_SPEED;
        }

        if tetra::input::is_key_down(ctx, Key::D) {
            self.position -= Position::unit_x().rotated_z(-self.rotation) * X_AXIS_MOVE_SPEED;
        }

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
