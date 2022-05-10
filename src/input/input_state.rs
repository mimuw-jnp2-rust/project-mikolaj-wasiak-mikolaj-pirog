use crate::graph::node::Node;
use petgraph::{Directed, Graph};
use std::error::Error;
use tetra::math::Vec2;
use tetra::Context;

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
        ctx: &mut Context,
        graph: &mut Graph<Node, (), Directed, u32>,
        position: Vec2<f32>,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            InputState::Add => {
                graph.add_node(Node::new(ctx, position)?);
            }
            InputState::Remove => {}
            InputState::Move => {}
        }
        Ok(())
    }
}
