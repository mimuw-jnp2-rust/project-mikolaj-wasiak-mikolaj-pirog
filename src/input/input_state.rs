use petgraph::{Directed, Graph};
use tetra::math::Vec2;

#[derive(PartialEq)]
pub enum InputState {
    Add,
    Remove,
    Move,
}

impl InputState {
    // TODO: Tu chyba powinniśmy dać jakoś inaczej zbiór wierzchołków, ale cokolwiek na razie
    pub fn on_left_click(
        &self,
        graph: &mut Graph<Vec2<f32>, (), Directed, u32>,
        position: Vec2<f32>,
    ) {
        match self {
            InputState::Add => {
                graph.add_node(position);
            }
            InputState::Remove => todo!(),
            InputState::Move => todo!(),
        }
    }
}
