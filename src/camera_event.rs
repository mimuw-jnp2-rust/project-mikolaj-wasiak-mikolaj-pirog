use std::error::Error;
use std::f32::consts::PI;

use tetra::input::Key;

use crate::graph::Position;
use crate::GameState;

pub const CAMERA_ZOOM_SPEED: f32 = 0.05;
const Y_AXIS_MOVE_SPEED: f32 = 10.;
const X_AXIS_MOVE_SPEED: f32 = 10.;

pub fn handle_camera_events(
    game_state: &mut GameState,
    event: tetra::Event,
) -> Result<(), Box<dyn Error>> {
    // Only y coordinate is accessed because x corresponds to horizontal move of mouse wheel.
    if let tetra::Event::MouseWheelMoved { amount } = &event {
        if amount.y > 0 {
            game_state.camera.scale += CAMERA_ZOOM_SPEED;
        } else {
            game_state.camera.scale -= CAMERA_ZOOM_SPEED;
        }
    }

    if let tetra::Event::KeyPressed { key: Key::W } = &event {
        game_state.camera.position +=
            Position::unit_y().rotated_z(-game_state.camera.rotation) * Y_AXIS_MOVE_SPEED;
    }

    if let tetra::Event::KeyPressed { key: Key::S } = &event {
        game_state.camera.position -=
            Position::unit_y().rotated_z(-game_state.camera.rotation) * Y_AXIS_MOVE_SPEED;
    }

    if let tetra::Event::KeyPressed { key: Key::A } = &event {
        game_state.camera.position +=
            Position::unit_x().rotated_z(-game_state.camera.rotation) * Y_AXIS_MOVE_SPEED;
    }

    if let tetra::Event::KeyPressed { key: Key::D } = &event {
        game_state.camera.position -=
            Position::unit_x().rotated_z(-game_state.camera.rotation) * Y_AXIS_MOVE_SPEED;
    }

    game_state.camera.update();

    if let tetra::Event::Resized { width, height } = event {
        game_state.scaler.set_outer_size(width, height);
    }

    Ok(())
}
