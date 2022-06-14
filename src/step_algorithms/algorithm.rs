use std::collections::VecDeque;

use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::Directed;

use crate::graph::node::{Node, NodeState};

use tetra::Context;

use crate::graph::{edge::Edge, node::VisibleNode};

use super::Timer;

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
    fn run_algorithm(self, graph: &mut GenericGraph<N, E>, start_idx: NodeIndex) -> AlgorithmResult;
}

pub struct AlgorithmResult {
    start_idx: NodeIndex,
    steps: VecDeque<AlgorithmStep>,
    timer: Timer,
}

impl AlgorithmResult {
    pub fn from_alg(steps: VecDeque<AlgorithmStep>, start_idx: NodeIndex) -> AlgorithmResult {
        let timer = Timer::new(0.5, true);
        AlgorithmResult {
            start_idx,
            steps,
            timer,
        }
    }

    pub fn get_steps(&self) -> &VecDeque<AlgorithmStep> {
        &self.steps
    }

    fn timer(&self) -> &Timer {
        &self.timer
    }

    fn timer_mut(&mut self) -> &mut Timer {
        &mut self.timer
    }

    fn start_timer(&mut self) {
        self.timer_mut().start();
    }

    pub fn update(&mut self, ctx: &mut Context, graph: &mut GenericGraph<VisibleNode, Edge>) {
        if self.timer_mut().update(ctx) {
            println!("timer ticked");
            if let Some(alg_step) = self.steps.pop_front() {
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

                if let Some(node) = graph.node_weight_mut(self.start_idx) {
                    node.set_ignore_force(false)
                }
            }
        }
    }

    fn turn_off_start_node_gravity(&mut self, graph: &mut GenericGraph<VisibleNode, Edge>) {
        if let Some(node) = graph.node_weight_mut(self.start_idx) {
            node.set_ignore_force(true)
        }
    }

    pub fn show_algorithm(&mut self, graph: &mut GenericGraph<VisibleNode, Edge>) {
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
}
