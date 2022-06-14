use egui_tetra::egui::CtxRef;
use egui_tetra::State;
use petgraph::{
    graph::NodeIndex,
    Directed,
    EdgeDirection::{Incoming, Outgoing},
};
use tetra::math::Vec2;
use tetra::Context;

use edge::Edge;
use node::VisibleNode;

use self::{gravity::{PullForceConfig, PushForceConfig}, node::{Node, NodeState}};

pub mod edge;
pub mod gravity;
pub mod node;
pub mod random;

pub type Graph = petgraph::Graph<VisibleNode, Edge, Directed, u32>;
pub type Position = Vec2<f32>;

pub trait GraphOnCanvas {
    fn get_node_from_point(&self, point: Position) -> Option<NodeIndex<u32>>;
    fn connect_nodes(&mut self, ctx: &mut Context, from: NodeIndex, to: NodeIndex);

    fn move_node(&mut self, ctx: &mut Context, idx: NodeIndex, position: Position);

    fn update_edges_position(
        &mut self,
        ctx: &mut Context,
        idx: NodeIndex,
        position: Position,
        direction: petgraph::EdgeDirection,
    );

    fn push_force(&mut self, push_conf: &PushForceConfig);
    fn pull_force(&mut self, pull_conf: &PullForceConfig);

    fn reset_state(&mut self);

    fn update(
        &mut self,
        ctx: &mut Context,
        egui_ctx: &CtxRef,
        push_conf: &PushForceConfig,
        pull_conf: &PullForceConfig,
    );

    fn draw(&mut self, mouse_position: Vec2<f32>, ctx: &mut Context, egui_ctx: &CtxRef);
}

impl GraphOnCanvas for Graph {
    fn get_node_from_point(&self, point: Position) -> Option<NodeIndex<u32>> {
        // Reversing to select node that is on top
        self.node_indices().rev().find(|idx| {
            self.node_weight(*idx)
                .map_or(false, |node| node.contains(point))
        })
    }

    // Wrapper for add_edge function
    fn connect_nodes(&mut self, ctx: &mut Context, from: NodeIndex, to: NodeIndex) {
        let edge = Edge::new(
            ctx,
            self.node_weight(from)
                .map_or(Position::zero(), |node| node.position()),
            self.node_weight(to)
                .map_or(Position::zero(), |node| node.position()),
        );

        self.add_edge(from, to, edge);
        println!("Connecting {} -> {}", from.index(), to.index());
    }

    fn move_node(&mut self, ctx: &mut Context, idx: NodeIndex, to: Position) {
        if let Some(node) = self.node_weight_mut(idx) {
            node.set_position(to);
        }

        self.update_edges_position(ctx, idx, to, Outgoing);
        self.update_edges_position(ctx, idx, to, Incoming);
    }

    fn update_edges_position(
        &mut self,
        ctx: &mut Context,
        idx: NodeIndex,
        position: Position,
        direction: petgraph::EdgeDirection,
    ) {
        let mut neigh_iter = self.neighbors_directed(idx, direction).detach();

        while let Some((edge_idx, neigh_idx)) = neigh_iter.next(self) {
            let neigh_pos_opt = self
                .node_weight(neigh_idx)
                .map(|neigh_data| neigh_data.position());
            let edge_weight_opt = self.edge_weight_mut(edge_idx);
            if let (Some(neigh_pos), Some(edge_data)) = (neigh_pos_opt, edge_weight_opt) {
                match direction {
                    Outgoing => edge_data.update_position(ctx, position, neigh_pos),
                    Incoming => edge_data.update_position(ctx, neigh_pos, position),
                };
            }
        }
    }

    fn push_force(&mut self, push_conf: &PushForceConfig) {
        for idx in self.node_indices() {
            for other_idx in self.node_indices() {
                if idx == other_idx {
                    continue;
                }

                self.node_weight(other_idx)
                    .map(|other_node| other_node.position())
                    .map(|other_pos| {
                        self.node_weight_mut(idx)
                            .map(|node| node.push_away_from_point(other_pos, push_conf))
                    });
            }
        }
    }

    fn pull_force(&mut self, pull_conf: &PullForceConfig) {
        for idx in self.node_indices() {
            let mut result = Position::zero();

            for edge_in in self.edges_directed(idx, Incoming) {
                result -= edge_in.weight().calculate_pull_force(pull_conf);
            }

            for edge_out in self.edges_directed(idx, Outgoing) {
                result += edge_out.weight().calculate_pull_force(pull_conf);
            }

            if let Some(node) = self.node_weight_mut(idx) {
                node.add_force(result)
            }
        }
    }

    fn update(
        &mut self,
        ctx: &mut Context,
        egui_ctx: &CtxRef,
        push_conf: &PushForceConfig,
        pull_conf: &PullForceConfig,
    ) {
        self.push_force(push_conf);
        self.pull_force(pull_conf);

        for node_idx in self.node_indices() {
            self.node_weight_mut(node_idx)
                .map(|node| {
                    node.consume_force(ctx);
                    node.position()
                })
                .map(|pos| self.move_node(ctx, node_idx, pos));
        }
    }

    fn draw(&mut self, mouse_position: Vec2<f32>, ctx: &mut Context, egui_ctx: &CtxRef) {
        for edge in self.edge_weights_mut() {
            edge.draw(ctx, egui_ctx);
        }

        for node in self.node_weights_mut() {
            node.draw(ctx, egui_ctx, mouse_position);
        }
    }

    fn reset_state(&mut self) {
        for node in self.node_weights_mut() {
            node.set_ignore_force(false);
            node.set_state(NodeState::NotVisited);
        }
        for edge in self.edge_weights_mut() {
            edge.reset_state();
        }
    }
}
