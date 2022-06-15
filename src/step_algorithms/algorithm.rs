use std::{any::Any, collections::VecDeque, fmt::Debug};

use dyn_partial_eq::dyn_partial_eq;
use petgraph::graph::NodeIndex;
use petgraph::Directed;

use crate::graph::{Graph, GraphOnCanvas};

use tetra::Context;

use super::Timer;

pub type GenericGraph<N, E> = petgraph::Graph<N, E, Directed, u32>;

#[dyn_partial_eq]
pub trait AlgorithmStep: Any + Debug {
    fn step(&self, graph: &mut Graph);
}

pub trait Algorithm<N, E> {
    fn run_algorithm(self, graph: &GenericGraph<N, E>, start_idx: NodeIndex) -> AlgorithmResult;
}

pub struct AlgorithmResult {
    start_idx: NodeIndex,
    steps: VecDeque<Box<dyn AlgorithmStep>>,
    timer: Timer,
}

impl AlgorithmResult {
    pub fn from_alg(
        steps: VecDeque<Box<dyn AlgorithmStep>>,
        start_idx: NodeIndex,
    ) -> AlgorithmResult {
        let timer = Timer::new(0.5, true);
        AlgorithmResult {
            start_idx,
            steps,
            timer,
        }
    }

    pub fn get_steps(&self) -> &VecDeque<Box<dyn AlgorithmStep>> {
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
                alg_step.step(graph);
            } else {
                self.timer_mut().stop();

                if let Some(node) = graph.node_weight_mut(self.start_idx) {
                    node.set_ignore_force(false)
                }
            }
        }
    }

    fn turn_off_start_node_gravity(&mut self, graph: &mut Graph) {
        if let Some(node) = graph.node_weight_mut(self.start_idx) {
            node.set_ignore_force(true)
        }
    }

    pub fn show_algorithm(&mut self, graph: &mut Graph) {
        graph.reset_state();

        // Allow node to move while the algorithm is being showcased
        for edge in graph.edge_weights_mut() {
            edge.disable_edge();
        }

        self.start_timer();
        self.turn_off_start_node_gravity(graph);
    }
}
