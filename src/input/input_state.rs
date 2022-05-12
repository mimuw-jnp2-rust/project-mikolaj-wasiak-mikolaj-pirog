use crate::graph::edge::Edge;
use crate::graph::node::NodeHighlight;
use crate::graph::GraphOnCanvas;
use crate::graph::Position;
use crate::graph::{node::Node, Graph};
use petgraph::graph::NodeIndex;
use std::error::Error;
use tetra::math::Vec2;
use tetra::Context;

#[derive(Default)]
pub struct ConnectData {
    from_node: Option<NodeIndex<u32>>,
}

pub enum InputState {
    Add,
    Remove,
    Move,
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
            InputState::Move => {}
            InputState::Connect(data) => match data.from_node {
                Some(from) => {
                    graph
                        .get_node_from_point(position)
                        .map(|to| -> Result<(), Box<dyn Error>> {
                            graph.add_edge(
                                from,
                                to,
                                Edge::new(
                                    ctx,
                                    graph
                                        .node_weight(from)
                                        .map_or(Position::zero(), |node| node.position()),
                                    graph
                                        .node_weight(to)
                                        .map_or(Position::zero(), |node| node.position()),
                                )?,
                            );
                            println!("Connecting {} -> {}", from.index(), to.index());
                            Ok(())
                        });
                    graph
                        .node_weight_mut(from)
                        .map(|node| node.set_highlight(NodeHighlight::Normal));

                    data.from_node = None;
                }
                None => {
                    data.from_node = graph.get_node_from_point(position);
                    data.from_node
                        .and_then(|idx| graph.node_weight_mut(idx))
                        .map(|node| node.set_highlight(NodeHighlight::Highlighted));
                }
            },
        }
        Ok(())
    }
}

impl PartialEq for InputState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InputState::Add, InputState::Add) => true,
            (InputState::Remove, InputState::Remove) => true,
            (InputState::Move, InputState::Move) => true,
            (InputState::Connect(_), InputState::Connect(_)) => true,
            _ => false,
        }
    }
}
