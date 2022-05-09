use tetra::ContextBuilder;

mod game_state;
pub use crate::game_state::GameState;

mod gui;
mod input;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", 640, 480)
        .show_mouse(true)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
