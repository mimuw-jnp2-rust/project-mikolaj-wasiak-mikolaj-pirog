use std::error::Error;

use egui_tetra::StateWrapper;
use tetra::ContextBuilder;

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub use crate::game_state::GameState;

mod camera_handling;
mod constants;
mod game_state;
mod graph;
mod input;
mod step_algorithms;
mod tetra_handling;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    ContextBuilder::new("Graph vis", SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .show_mouse(true)
        .quit_on_escape(true)
        .build()
        .unwrap()
        .run(|ctx| Ok(StateWrapper::new(GameState::new(ctx))))
}
