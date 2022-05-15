use std::error::Error;

use petgraph::graph::NodeIndex;
use tetra::math::Vec2;
use tetra::Context;

use crate::graph::node::NodeHighlight;
use crate::graph::GraphOnCanvas;
use crate::graph::{node::Node, Graph};

#[derive(Default)]
pub struct ConnectData {
    from_node: Option<NodeIndex<u32>>,
}

#[derive(Default)]
pub struct MoveData {
    selected_node: Option<NodeIndex<u32>>,
}

pub enum InputState {
    Add,
    Remove,
    Move(MoveData),
    Connect(ConnectData),
}

impl InputState {
    pub fn on_left_click(
        &mut self,
        ctx: &mut Context,
        graph: &mut Graph,
        position: Vec2<f32>,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            InputState::Add => {
                graph.add_node(Node::new(ctx, position)?);
            }
            InputState::Remove => {
                graph
                    .get_node_from_point(position)
                    .map(|idx| graph.remove_node(idx));
            }
            InputState::Move(data) => match data.selected_node {
                Some(node_idx) => {
                    data.selected_node = None;
                    if let Some(node) = graph.node_weight_mut(node_idx) {
                        node.set_ignore_force(false)
                    }
                }
                None => {
                    data.selected_node = graph.get_node_from_point(position);
                    if let Some(node_idx) = data.selected_node {
                        if let Some(node) = graph.node_weight_mut(node_idx) {
                            node.set_ignore_force(true)
                        }
                    }
                }
            },
            InputState::Connect(data) => match data.from_node {
                Some(from) => {
                    graph
                        .get_node_from_point(position)
                        .map(|to| graph.connect_nodes(ctx, from, to));
                    if let Some(node) = graph.node_weight_mut(from) {
                        node.set_highlight(NodeHighlight::Normal)
                    }

                    data.from_node = None;
                }
                None => {
                    data.from_node = graph.get_node_from_point(position);
                    if let Some(node) = data.from_node.and_then(|idx| graph.node_weight_mut(idx)) {
                        node.set_highlight(NodeHighlight::Highlighted)
                    }
                }
            },
        }
        Ok(())
    }

    pub fn on_mouse_drag(
        &mut self,
        ctx: &mut Context,
        graph: &mut Graph,
        position: Vec2<f32>,
    ) -> Result<(), Box<dyn Error>> {
        if let InputState::Move(data) = self {
            match data.selected_node {
                None => (),
                Some(node_idx) => graph.move_node(ctx, node_idx, position)?,
            }
        }
        Ok(())
    }
}

impl PartialEq for InputState {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (InputState::Add, InputState::Add)
                | (InputState::Remove, InputState::Remove)
                | (InputState::Move(_), InputState::Move(_))
                | (InputState::Connect(_), InputState::Connect(_))
        )
    }
}
