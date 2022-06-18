use std::collections::HashMap;
use std::collections::VecDeque;

use dyn_partial_eq::DynPartialEq;
use petgraph::graph::EdgeIndex;
use petgraph::graph::NodeIndex;
use petgraph::Direction;
use tetra::graphics::Color;

use super::StepAlgorithm;
use super::StepAlgorithmResult;
use crate::step_algorithms::step_algorithm::GenericGraph;
use crate::step_algorithms::step_algorithm::Step;

#[derive(PartialEq, Debug)]
pub enum NodeState {
    Visited,
    Queued,
    NotVisited,
}

#[derive(DynPartialEq, PartialEq, Debug)]
pub struct NodeStep {
    idx: NodeIndex,
    to_state: NodeState,
}

impl NodeStep {
    pub fn new(idx: NodeIndex, to_state: NodeState) -> NodeStep {
        NodeStep { idx, to_state }
    }
}

impl Step for NodeStep {
    fn apply_step(&self, graph: &mut crate::graph::Graph) {
        if let Some(node) = graph.node_weight_mut(self.idx) {
            node.set_color(match self.to_state {
                NodeState::Visited => Color::GREEN,
                NodeState::Queued => Color::rgb8(200, 200, 200),
                NodeState::NotVisited => Color::WHITE,
            });
        }
    }
}

#[derive(DynPartialEq, PartialEq, Debug)]
pub struct EdgeStep {
    idx: EdgeIndex,
}

impl EdgeStep {
    pub fn new(idx: EdgeIndex) -> EdgeStep {
        EdgeStep { idx }
    }
}

impl Step for EdgeStep {
    fn apply_step(&self, graph: &mut crate::graph::Graph) {
        if let Some(edge) = graph.edge_weight_mut(self.idx) {
            edge.enable_edge();
        }
    }
}

pub struct Dfs {
    steps: VecDeque<Box<dyn Step>>,
    states: HashMap<NodeIndex, NodeState>,
}

impl<N, E> StepAlgorithm<N, E> for Dfs {
    fn get_result(
        mut self,
        graph: &GenericGraph<N, E>,
        start_idx: NodeIndex,
    ) -> StepAlgorithmResult {
        self.dfs(graph, start_idx);
        StepAlgorithmResult::from_steps(self.steps, start_idx)
    }
}

impl Dfs {
    pub fn from_graph<N, E>(graph: &GenericGraph<N, E>) -> Dfs {
        let mut states = HashMap::new();
        for index in graph.node_indices() {
            states.insert(index, NodeState::NotVisited);
        }
        Dfs {
            steps: VecDeque::new(),
            states,
        }
    }

    fn dfs<N, E>(&mut self, graph: &GenericGraph<N, E>, start_idx: NodeIndex) {
        self.dfs_helper(graph, start_idx);
    }

    fn dfs_helper<N, E>(&mut self, graph: &GenericGraph<N, E>, node_index: NodeIndex) {
        self.steps
            .push_back(Box::new(NodeStep::new(node_index, NodeState::Queued)));

        self.states.insert(node_index, NodeState::Queued);

        let mut walker = graph
            .neighbors_directed(node_index, Direction::Outgoing)
            .detach();

        while let Some((edge_idx, other_node_idx)) = walker.next(graph) {
            if let Some(other_state) = self.states.get(&other_node_idx) {
                if matches!(other_state, NodeState::NotVisited) {
                    self.steps.push_back(Box::new(EdgeStep::new(edge_idx)));
                    self.dfs_helper(graph, other_node_idx);
                }
            }
        }

        self.steps
            .push_back(Box::new(NodeStep::new(node_index, NodeState::Visited)));

        self.states.insert(node_index, NodeState::Visited);
    }
}

#[cfg(test)]
mod tests {
    use super::{Dfs, GenericGraph};
    use crate::step_algorithms::{
        dfs::{EdgeStep, NodeState, NodeStep},
        step_algorithm::Step,
        StepAlgorithm,
    };
    use std::collections::VecDeque;

    #[test]
    fn small_test() {
        let mut graph = GenericGraph::<u32, u32>::new();
        let a = graph.add_node(1);
        let b = graph.add_node(2);
        let edge_idx = graph.add_edge(a, b, 0);

        let res = Dfs::from_graph(&graph).get_result(&graph, a);

        let mut desired = VecDeque::<Box<dyn Step>>::new();
        desired.push_back(Box::new(NodeStep::new(a, NodeState::Queued)));
        desired.push_back(Box::new(EdgeStep::new(edge_idx)));
        desired.push_back(Box::new(NodeStep::new(b, NodeState::Queued)));
        desired.push_back(Box::new(NodeStep::new(b, NodeState::Visited)));
        desired.push_back(Box::new(NodeStep::new(a, NodeState::Visited)));

        assert_eq!(res.get_steps(), &desired);
    }
}
