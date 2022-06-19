use std::{any::Any, collections::VecDeque, fmt::Debug};

use dyn_partial_eq::dyn_partial_eq;
use petgraph::graph::NodeIndex;
use petgraph::Directed;

use crate::graph::Graph;

use tetra::Context;

use super::Timer;

pub type GenericGraph<N, E> = petgraph::Graph<N, E, Directed, u32>;

#[dyn_partial_eq]
pub trait Step: Any + Debug {
    fn apply_step(&self, graph: &mut Graph);
}

pub trait StepAlgorithm<N, E> {
    fn get_result(self, graph: &GenericGraph<N, E>, start_idx: NodeIndex) -> StepAlgorithmResult;
}

pub struct StepAlgorithmResult {
    start_idx: NodeIndex,
    steps: VecDeque<Box<dyn Step>>,
    timer: Timer,
}

impl StepAlgorithmResult {
    pub fn from_steps(steps: VecDeque<Box<dyn Step>>, start_idx: NodeIndex) -> StepAlgorithmResult {

        let timer = Timer::new(0.2, true);
        StepAlgorithmResult {
            start_idx,
            steps,
            timer,
        }
    }

    pub fn get_steps(&self) -> &VecDeque<Box<dyn Step>> {
        &self.steps
    }

    fn timer_mut(&mut self) -> &mut Timer {
        &mut self.timer
    }

    fn start_timer(&mut self) {
        self.timer_mut().start();
    }

    pub fn update(&mut self, ctx: &mut Context, graph: &mut Graph) {
        if self.timer_mut().update(ctx) {
            if let Some(alg_step) = self.steps.pop_front() {
                alg_step.apply_step(graph);
            } else {
                self.timer_mut().stop();

                self.toggle_start_node_gravity_ignoring(graph, false);
            }
        }
    }

    fn toggle_start_node_gravity_ignoring(&mut self, graph: &mut Graph, on: bool) {
        if let Some(node) = graph.node_weight_mut(self.start_idx) {
            node.set_ignore_force(on)
        }
    }

    pub fn show_algorithm(&mut self, graph: &mut Graph) {
        // Allow node to move while the algorithm is being showcased
        for edge in graph.edge_weights_mut() {
            edge.disable_edge();
        }

        self.start_timer();
        self.toggle_start_node_gravity_ignoring(graph, true);
    }
}
