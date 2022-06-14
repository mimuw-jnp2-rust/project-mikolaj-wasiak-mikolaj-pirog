use rand::seq::IteratorRandom;
use tetra::Context;

use super::{Graph, GraphOnCanvas, Position, VisibleNode};

// TODO: Animate that
pub fn generate(ctx: &mut Context, node_count: u32, edge_count: u32) -> Graph {
    println!(
        "Generating graph with {} nodes and {} edges",
        node_count, edge_count
    );
    let mut graph = Graph::new();
    for _ in 0..node_count {
        let weight = VisibleNode::new(ctx, Position::zero());
        graph.add_node(weight);
    }
    let mut rng = rand::thread_rng();
    for _ in 0..edge_count {
        let a_opt = graph.node_indices().choose(&mut rng);
        let b_opt = graph.node_indices().choose(&mut rng);
        if let (Some(a), Some(b)) = (a_opt, b_opt) {
            if a != b {
                graph.connect_nodes(ctx, a, b);
            }
        }
    }
    graph
}
