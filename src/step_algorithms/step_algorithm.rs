use std::{any::Any, collections::VecDeque, fmt::Debug};

use dyn_partial_eq::dyn_partial_eq;
use petgraph::graph::NodeIndex;
use petgraph::{Directed, EdgeType, Graph, Undirected};

use tetra::Context;

use super::Timer;

#[dyn_partial_eq]
pub trait Step: Any + Debug {
    fn apply_step(&self, graph: &mut crate::graph::Graph);
}

pub trait StepAlgorithm {
    fn run<N, E, D: EdgeType>(&mut self, graph: &Graph<N, E, D>, start_idx: NodeIndex);
    fn result(self) -> StepAlgorithmResult;
}

pub trait UndirectedStepAlgorithm {
    fn run<N, E>(&mut self, graph: &Graph<N, E, Undirected>, start_idx: NodeIndex);
    fn result(self) -> StepAlgorithmResult;
}

pub trait DirectedStepAlgorithm {
    fn run<N, E>(&mut self, graph: &Graph<N, E, Directed>, start_idx: NodeIndex);
    fn result(self) -> StepAlgorithmResult;
}

pub struct StepAlgorithmResult {
    steps: VecDeque<Box<dyn Step>>,
    timer: Timer,
}

impl StepAlgorithmResult {
    pub fn from_steps(steps: VecDeque<Box<dyn Step>>) -> StepAlgorithmResult {
        let timer = Timer::new(0.3, true);
        StepAlgorithmResult { steps, timer }
    }

    pub fn steps(&self) -> &VecDeque<Box<dyn Step>> {
        &self.steps
    }

    fn timer_mut(&mut self) -> &mut Timer {
        &mut self.timer
    }

    fn start_timer(&mut self) {
        self.timer_mut().start();
    }

    pub fn update(&mut self, ctx: &mut Context, graph: &mut crate::graph::Graph) {
        if self.timer_mut().update(ctx) {
            if let Some(alg_step) = self.steps.pop_front() {
                alg_step.apply_step(graph);
            } else {
                self.timer_mut().stop();
            }
        }
    }

    pub fn show_algorithm(&mut self, graph: &mut crate::graph::Graph) {
        // Allow node to move while the algorithm is being showcased
        for edge in graph.edge_weights_mut() {
            edge.disable();
        }

        self.start_timer();
    }
}
