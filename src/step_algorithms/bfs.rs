use std::collections::HashMap;
use std::collections::VecDeque;

use dyn_partial_eq::DynPartialEq;
use petgraph::graph::EdgeIndex;
use petgraph::graph::NodeIndex;
use petgraph::Direction;
use petgraph::EdgeType;
use petgraph::Graph;
use tetra::graphics::Color;

use super::StepAlgorithm;
use super::StepAlgorithmResult;
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

pub struct Bfs {
    steps: VecDeque<Box<dyn Step>>,
    states: HashMap<NodeIndex, NodeState>,
}

impl StepAlgorithm for Bfs {
    fn result<N, E, D: EdgeType>(
        mut self,
        graph: &Graph<N, E, D>,
        start_idx: NodeIndex,
    ) -> StepAlgorithmResult {
        self.bfs(graph, start_idx);
        StepAlgorithmResult::from_steps(self.steps, start_idx)
    }
}

impl Bfs {
    pub fn from_graph<N, E, D: EdgeType>(graph: &Graph<N, E, D>) -> Bfs {
        let mut states = HashMap::new();
        for index in graph.node_indices() {
            states.insert(index, NodeState::NotVisited);
        }
        Bfs {
            steps: VecDeque::new(),
            states,
        }
    }

    fn bfs<N, E, D: EdgeType>(&mut self, graph: &Graph<N, E, D>, start_idx: NodeIndex) {
        let mut q = VecDeque::<NodeIndex>::new();
        q.push_back(start_idx);
        self.steps
            .push_back(Box::new(NodeStep::new(start_idx, NodeState::Queued)));
        self.states.insert(start_idx, NodeState::Queued);

        while let Some(idx) = q.pop_front() {
            let mut walker = graph.neighbors_directed(idx, Direction::Outgoing).detach();
            while let Some((edge_idx, other_node_idx)) = walker.next(graph) {
                if let Some(other_state) = self.states.get(&other_node_idx) {
                    if matches!(other_state, NodeState::NotVisited) {
                        self.steps.push_back(Box::new(EdgeStep::new(edge_idx)));
                        self.steps
                            .push_back(Box::new(NodeStep::new(other_node_idx, NodeState::Queued)));

                        self.states.insert(other_node_idx, NodeState::Queued);
                        q.push_back(other_node_idx);
                    }
                }
            }
            self.states.insert(idx, NodeState::Visited);
            self.steps
                .push_back(Box::new(NodeStep::new(idx, NodeState::Visited)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Bfs;
    use crate::step_algorithms::{
        bfs::{EdgeStep, NodeState, NodeStep},
        step_algorithm::Step,
        StepAlgorithm,
    };
    use std::collections::VecDeque;

    #[test]
    fn small_test() {
        let mut graph = petgraph::Graph::<u32, u32, petgraph::Directed, u32>::new();
        let a = graph.add_node(1);
        let b = graph.add_node(2);
        let edge_idx = graph.add_edge(a, b, 0);

        let res = Bfs::from_graph(&graph).result(&graph, a);

        let mut desired = VecDeque::<Box<dyn Step>>::new();
        desired.push_back(Box::new(NodeStep::new(a, NodeState::Queued)));
        desired.push_back(Box::new(EdgeStep::new(edge_idx)));
        desired.push_back(Box::new(NodeStep::new(b, NodeState::Queued)));
        desired.push_back(Box::new(NodeStep::new(a, NodeState::Visited)));
        desired.push_back(Box::new(NodeStep::new(b, NodeState::Visited)));

        assert_eq!(res.steps(), &desired);
    }
}
