use crate::game_state::GameState;
use tetra::math::Vec2;

pub trait InputState {
    fn on_left_click(self, game_state: &mut GameState, position: Vec2<f32>);
}

#[derive(PartialEq)]
pub struct AddState {}

impl InputState for AddState {
    fn on_left_click(self, game_state: &mut GameState, position: Vec2<f32>) {
        game_state.graph.add_node(position);
    }
}
