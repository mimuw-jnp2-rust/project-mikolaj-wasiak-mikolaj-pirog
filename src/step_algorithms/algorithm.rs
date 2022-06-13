use std::collections::VecDeque;

use petgraph::Directed;
use petgraph::graph::{EdgeIndex, NodeIndex};
use tetra::Context;

use crate::graph::node::{Node, VisibleNode, NodeState};
use crate::graph::edge::Edge;
use crate::step_algorithms::timer::Timer;

pub type GenericGraph<N, E> = petgraph::Graph<N, E, Directed, u32>;

#[derive(PartialEq, Debug)]
pub struct NodeStep {
    idx: NodeIndex,
    to_state: NodeState,
}

impl NodeStep {
    pub fn new(idx: NodeIndex, to_state: NodeState) -> NodeStep {
        NodeStep { idx, to_state }
    }
}

#[derive(PartialEq, Debug)]
pub struct EdgeStep {
    idx: EdgeIndex,
}

impl EdgeStep {
    pub fn new(idx: EdgeIndex) -> EdgeStep {
        EdgeStep { idx }
    }

    // This is unused now, but will be later.
    pub fn _idx(&self) -> EdgeIndex {
        self.idx
    }
}

#[derive(PartialEq, Debug)]
pub enum AlgorithmStep {
    Node(NodeStep),
    Edge(EdgeStep),
}

pub trait Algorithm<N: Node, E> {
    fn start_idx(&self) -> NodeIndex;

    fn steps(&self) -> &VecDeque<AlgorithmStep>;

    fn steps_mut(&mut self) -> &mut VecDeque<AlgorithmStep>;

    fn add_step(&mut self, algo_step: AlgorithmStep) {
        self.steps_mut().push_back(algo_step);
    }

    fn reset_algorithm(&mut self, graph: &mut GenericGraph<N, E>) {
        for node in graph.node_weights_mut() {
            node.set_state(NodeState::NotVisited);
        }
        
    }

    fn run_algorithm(&mut self, graph: &mut GenericGraph<N, E>);
}

pub trait VisibleAlgorithm: Algorithm<VisibleNode, Edge> {
    fn timer(&self) -> &Timer;

    fn timer_mut(&mut self) -> &mut Timer;

    fn start_timer(&mut self) {
        self.timer_mut().start();
    }

    fn update(&mut self, ctx: &mut Context, graph: &mut GenericGraph<VisibleNode, Edge>) {
        if self.timer_mut().update(ctx) {
            println!("timer ticked");
            if let Some(alg_step) = self.steps_mut().pop_front() {
                match alg_step {
                    AlgorithmStep::Node(step) => {
                        if let Some(node) = graph.node_weight_mut(step.idx) {
                            node.set_state(step.to_state)
                        }
                    }
                    AlgorithmStep::Edge(step) => {
                        if let Some(edge) = graph.edge_weight_mut(step.idx) {
                            edge.enable_edge();
                        }
                    }
                }
            } else {
                self.timer_mut().stop();

                if let Some(node) = graph.node_weight_mut(self.start_idx()) {
                    node.set_ignore_force(false)
                }
            }
        }
    }

    fn turn_off_start_node_gravity(&mut self, graph: &mut GenericGraph<VisibleNode, Edge>) {
        if let Some(node) = graph.node_weight_mut(self.start_idx()) {
            node.set_ignore_force(true)
        }
    }

    fn show_algorithm(&mut self, graph: &mut GenericGraph<VisibleNode, Edge>) {
        for node in graph.node_weights_mut() {
            node.set_state(NodeState::NotVisited);
        }

        // Allow node to move while the algorithm is being showcased
        for edge in graph.edge_weights_mut() {
            edge.disable_edge();
        }

        self.start_timer();
        self.turn_off_start_node_gravity(graph);
    }

    fn reset_algorithm(&mut self, graph: &mut GenericGraph<VisibleNode, Edge>) {
        Algorithm::reset_algorithm(self, graph);
        
        for edge in graph.edge_weights_mut() {
            edge.enable_edge();
        }
    }
} 
