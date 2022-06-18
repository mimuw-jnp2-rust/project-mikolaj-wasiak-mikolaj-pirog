use petgraph::graph::NodeIndex;
use tetra::graphics::text::Font;
use tetra::math::Vec2;
use tetra::Context;

use crate::graph::node::NodeHighlight;
use crate::graph::GraphOnCanvas;
use crate::graph::{node::Node, Graph};

#[derive(Default)]
pub struct StateData {
    pub selected_node: Option<NodeIndex<u32>>,
}

pub enum InputState {
    Add,
    Remove,
    Move(StateData),
    Connect(StateData),
    Select(StateData),
}

impl InputState {
    pub fn on_left_click(
        &mut self,
        ctx: &mut Context,
        graph: &mut Graph,
        position: Vec2<f32>,
        font: Font,
    ) {
        match self {
            InputState::Add => {
                graph.add_node(Node::new(ctx, position, font));
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
            InputState::Connect(data) => match data.selected_node {
                Some(from) => {
                    if let Some(to) = graph.get_node_from_point(position) {
                        graph.connect_nodes(ctx, from, to)
                    }
                    if let Some(node) = graph.node_weight_mut(from) {
                        node.set_highlight(NodeHighlight::Normal)
                    }

                    data.selected_node = None;
                }
                None => {
                    data.selected_node = graph.get_node_from_point(position);
                    if let Some(node) = data
                        .selected_node
                        .and_then(|idx| graph.node_weight_mut(idx))
                    {
                        node.set_highlight(NodeHighlight::Highlighted)
                    }
                }
            },
            InputState::Select(data) => {
                if let Some(idx) = data.selected_node {
                    if let Some(node) = graph.node_weight_mut(idx) {
                        node.set_highlight(NodeHighlight::Normal)
                    }
                }
                data.selected_node = graph.get_node_from_point(position);
                if let Some(idx) = data.selected_node {
                    if let Some(node) = graph.node_weight_mut(idx) {
                        node.set_highlight(NodeHighlight::Highlighted)
                    }
                }
            }
        }
    }

    pub fn on_mouse_drag(&mut self, ctx: &mut Context, graph: &mut Graph, position: Vec2<f32>) {
        if let InputState::Move(data) = self {
            match data.selected_node {
                None => (),
                Some(node_idx) => graph.move_node(ctx, node_idx, position),
            }
        }
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
                | (InputState::Select(_), InputState::Select(_))
        )
    }
}
