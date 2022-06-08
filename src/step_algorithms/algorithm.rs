use std::collections::VecDeque;

use petgraph::graph::{EdgeIndex, NodeIndex};
use tetra::Context;

use crate::graph::node::NodeState;
use crate::graph::Graph;
use crate::step_algorithms::timer::Timer;


pub struct NodeStep {
    idx: NodeIndex,
    to_state: NodeState,
}

impl NodeStep {
    pub fn new(idx: NodeIndex, to_state: NodeState) -> NodeStep {
        NodeStep { idx, to_state }
    }
}

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

pub enum AlgorithmStep {
    Node(NodeStep),
    Edge(EdgeStep),
}

pub trait Algorithm {
   /* fn new(start_idx: NodeIndex) -> Algorithm {
        Algorithm {
            steps: VecDeque::new(),
            timer: Timer::new(1., true),
            start_idx,
        }
    }*/

    fn update(&mut self, ctx: &mut Context, graph: &mut Graph) {
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

    fn start_idx(&self) -> NodeIndex;

    fn timer(&self) -> &Timer;

    fn timer_mut(&mut self) -> &mut Timer;

    fn steps(&self) -> &Vec<AlgorithmStep>;

    fn steps_mut(&mut self) -> &mut Vec<AlgorithmStep>;

    fn add_step(&mut self, algo_step: AlgorithmStep) {
        self.steps_mut().push_back(algo_step);
    }

    fn start_timer(&mut self) {
        self.timer().start();
    }

    fn turn_off_start_node_gravity(&mut self, graph: &mut Graph) {
        if let Some(node) = graph.node_weight_mut(self.start_idx()) {
            node.set_ignore_force(true)
        }
    }

    fn run_algorithm(&self);
}
