use petgraph::{
    graph::NodeIndex,
    Directed,
    EdgeDirection::{Incoming, Outgoing},
};
use std::error::Error;
use tetra::Context;

pub mod edge;
pub mod node;
use edge::Edge;
use node::Node;

pub type Graph = petgraph::Graph<Node, Edge, Directed, u32>;
pub type Position = tetra::math::Vec2<f32>;

pub trait GraphOnCanvas {
    fn get_node_from_point(&self, point: Position) -> Option<NodeIndex<u32>>;
    fn connect_nodes(
        &mut self,
        ctx: &mut Context,
        from: NodeIndex,
        to: NodeIndex,
    ) -> Result<(), Box<dyn Error>>;

    fn update_node_position(
        &mut self,
        ctx: &mut Context,
        idx: NodeIndex,
        position: Position,
    ) -> Result<(), Box<dyn Error>>;

    fn update_edges_position(
        &mut self,
        ctx: &mut Context,
        idx: NodeIndex,
        position: Position,
        direction: petgraph::EdgeDirection,
    ) -> Result<(), Box<dyn Error>>;

    fn update(
        &mut self,
        ctx: &mut tetra::Context,
        egui_ctx: &egui_tetra::egui::CtxRef,
    ) -> Result<(), Box<dyn Error>>;
}

impl GraphOnCanvas for Graph {
    // Wrapper for add_edge function
    fn connect_nodes(
        &mut self,
        ctx: &mut Context,
        from: NodeIndex,
        to: NodeIndex,
    ) -> Result<(), Box<dyn Error>> {
        let edge = Edge::new(
            ctx,
            self.node_weight(from)
                .map_or(Position::zero(), |node| node.position()),
            self.node_weight(to)
                .map_or(Position::zero(), |node| node.position()),
        )?;
        self.add_edge(from, to, edge);
        println!("Connecting {} -> {}", from.index(), to.index());
        Ok(())
    }

    fn get_node_from_point(&self, point: Position) -> Option<NodeIndex<u32>> {
        // Reversing to select node that is on top
        self.node_indices().rev().find(|idx| {
            self.node_weight(*idx)
                .map_or(false, |node| node.contains(point))
        })
    }

    fn update_edges_position(
        &mut self,
        ctx: &mut Context,
        idx: NodeIndex,
        position: Position,
        direction: petgraph::EdgeDirection,
    ) -> Result<(), Box<dyn Error>> {
        let mut neigh_iter = self.neighbors_directed(idx, direction).detach();
        while let Some((edge_idx, neigh_idx)) = neigh_iter.next(self) {
            if let Some((neigh_pos, edge_data)) = self
                .node_weight(neigh_idx)
                .map(|neigh_data| neigh_data.position())
                .zip(self.edge_weight_mut(edge_idx))
            {
                match direction {
                    Outgoing => edge_data.update_position(ctx, position, neigh_pos)?,
                    Incoming => edge_data.update_position(ctx, neigh_pos, position)?,
                }
            }
        }
        Ok(())
    }

    fn update_node_position(
        &mut self,
        ctx: &mut Context,
        idx: NodeIndex,
        position: Position,
    ) -> Result<(), Box<dyn Error>> {
        self.node_weight_mut(idx).map(|node| {
            node.set_position(position);
        });
        self.update_edges_position(ctx, idx, position, Outgoing)?;
        self.update_edges_position(ctx, idx, position, Incoming)?;

        Ok(())
    }

    fn update(
        &mut self,
        ctx: &mut tetra::Context,
        egui_ctx: &egui_tetra::egui::CtxRef,
    ) -> Result<(), Box<dyn Error>> {
        for idx in self.node_indices() {
            for other_idx in self.node_indices() {
                if idx == other_idx {
                    continue;
                }
                self.node_weight(other_idx)
                    .map(|other_node| other_node.position())
                    .map(|other_pos| {
                        self.node_weight_mut(idx)
                            .map(|node| node.repel_from_point(other_pos))
                    });
            }
        }
        for node in self.node_weights_mut() {
            node.update(ctx, egui_ctx)?;
        }
        Ok(())
    }
}
