use egui_tetra::StateWrapper;
use game_state::{SCREEN_HEIGHT, SCREEN_WIDTH};
use std::error::Error;
use tetra::ContextBuilder;

mod game_state;
pub use crate::game_state::GameState;

mod graph;
mod input;

fn main() -> Result<(), Box<dyn Error>> {
    ContextBuilder::new("Graph vis", SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .show_mouse(true)
        .quit_on_escape(true)
        .build()?
        .run(|ctx| Ok(StateWrapper::new(GameState::new(ctx)?)))
}
