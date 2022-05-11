use crate::graph::node::Node;
use petgraph::{Directed, Graph};
use std::error::Error;
use tetra::input::get_mouse_position;
use tetra::math::Vec2;
use tetra::Context;

#[derive(PartialEq)]
pub enum InputState {
    Add,
    Remove,
    Move,
}

impl InputState {
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
            InputState::Remove => {
                graph
                    .node_indices()
                    .find(|idx| {
                        graph
                            .node_weight(*idx)
                            .map_or(false, |node| node.contains(get_mouse_position(ctx)))
                    })
                    .map(|idx| graph.remove_node(idx));
            }
            InputState::Move => {}
        }
        Ok(())
    }
}
