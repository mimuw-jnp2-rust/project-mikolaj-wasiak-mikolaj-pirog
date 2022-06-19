use petgraph::graph::NodeIndex;
use rand::seq::SliceRandom;

use tetra::graphics::text::Font;
use tetra::Context;

use crate::graph::node::Node;

use super::{Graph, GraphOnCanvas, Position};

// TODO: Animate that
pub fn generate(ctx: &mut Context, node_count: u32, edge_count: u32, font: Font) -> Graph {
    println!(
        "Generating graph with {} nodes and {} edges",
        node_count, edge_count
    );
    let mut graph = Graph::new();
    for _ in 0..node_count {
        let weight = Node::new(ctx, Position::zero(), font.clone());
        graph.add_node(weight);
    }
    let mut rng = rand::thread_rng();
    let indecies_weight = graph
        .node_indices()
        .map(|idx| -> (NodeIndex, u32) {
            (
                idx,
                edge_count - graph.neighbors(idx.clone()).count() as u32,
            )
        })
        .collect::<Vec<(NodeIndex, u32)>>();
    for _ in 0..edge_count {
        let a_res = indecies_weight.choose_weighted(&mut rng, |idx| idx.1);
        let b_res = indecies_weight.choose_weighted(&mut rng, |idx| idx.1);
        if let (Ok(a), Ok(b)) = (a_res, b_res) {
            if a != b {
                graph.connect_nodes(ctx, a.0.clone(), b.0.clone());
            }
        }
    }
    graph
}
