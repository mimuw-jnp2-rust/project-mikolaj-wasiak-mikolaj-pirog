use rand::prelude::*;
use rand::seq::IteratorRandom;
use std::error::Error;
use tetra::Context;

use super::{Graph, GraphOnCanvas, VisibleNode, Position};

// TODO: Button to create random graph

pub fn random(
    node_count: u32,
    edge_count: u32,
    ctx: &mut Context,
) -> Result<Graph, Box<dyn Error>> {
    let mut g = Graph::new();
    for _ in [0..node_count] {
        let weight = VisibleNode::new(ctx, Position::zero())?;
        g.add_node(weight);
    }
    let mut rng = rand::thread_rng();
    for _ in [0..edge_count] {
        let a_opt = g.node_indices().choose(&mut rng);
        let b_opt = g.node_indices().choose(&mut rng);
        if let (Some(a), Some(b)) = (a_opt, b_opt) {
            if a != b {
                g.connect_nodes(ctx, a, b)?;
            }
        }
    }
    Ok(g)
}
