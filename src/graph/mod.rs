use petgraph::{graph::NodeIndex, Directed};
pub mod edge;
pub mod node;
use edge::Edge;
use node::Node;

pub type Graph = petgraph::Graph<Node, Edge, Directed, u32>;
pub type Position = tetra::math::Vec2<f32>;

pub trait GraphOnCanvas {
    fn get_node_from_point(&self, point: Position) -> Option<NodeIndex<u32>>;
}

impl GraphOnCanvas for Graph {
    fn get_node_from_point(&self, point: Position) -> Option<NodeIndex<u32>> {
        // Reversing to select node that is on top
        self.node_indices().rev().find(|idx| {
            self.node_weight(*idx)
                .map_or(false, |node| node.contains(point))
        })
    }
}
