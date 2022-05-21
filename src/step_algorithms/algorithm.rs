use std::collections::VecDeque;

use petgraph::graph::{EdgeIndex, NodeIndex};
use tetra::Context;

use crate::graph::node::NodeState;
use crate::graph::Graph;
use crate::step_algorithms::timer::Timer;

pub struct Algorithm {
    pub(crate) steps: VecDeque<AlgorithmStep>,
    pub(crate) timer: Timer,
    pub(crate) start_idx: NodeIndex,
}

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
    pub(crate) idx: EdgeIndex,
}

pub enum AlgorithmStep {
    Node(NodeStep),
    Edge(EdgeStep),
}

impl Algorithm {
    pub fn new(start_idx: NodeIndex) -> Algorithm {
        Algorithm {
            steps: VecDeque::new(),
            timer: Timer::new(1., true),
            start_idx,
        }
    }

    pub fn update(&mut self, ctx: &mut Context, graph: &mut Graph) {
        if self.timer.update(ctx) {
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
                self.timer.stop();

                if let Some(node) = graph.node_weight_mut(self.start_idx) {
                    node.set_ignore_force(false)
                }
            }
        }
    }
}
