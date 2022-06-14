use std::collections::VecDeque;

use petgraph::graph::NodeIndex;
use petgraph::Direction;

use super::Algorithm;
use super::AlgorithmResult;
use crate::graph::node::{Node, NodeState};
use crate::step_algorithms::algorithm::GenericGraph;
use crate::step_algorithms::algorithm::{AlgorithmStep, EdgeStep, NodeStep};

pub struct Dfs {
    steps: VecDeque<AlgorithmStep>,
}

impl<N: Node, E> Algorithm<N, E> for Dfs {
    fn run_algorithm(
        mut self,
        graph: &mut GenericGraph<N, E>,
        start_idx: NodeIndex,
    ) -> AlgorithmResult {
        self.dfs(graph, start_idx);
        AlgorithmResult::from_alg(self.steps, start_idx)
    }
}

impl Dfs {
    pub fn new() -> Dfs {
        Dfs {
            steps: VecDeque::new(),
        }
    }

    fn dfs<N: Node, E>(&mut self, graph: &mut GenericGraph<N, E>, start_idx: NodeIndex) {
        self.dfs_helper(graph, start_idx);
    }

    fn dfs_helper<N, E>(&mut self, graph: &mut GenericGraph<N, E>, node_index: NodeIndex)
    where
        N: Node,
    {
        self.steps.push_back(AlgorithmStep::Node(NodeStep::new(
            node_index,
            NodeState::Queued,
        )));

        if let Some(node) = graph.node_weight_mut(node_index) {
            node.set_state(NodeState::Queued)
        }

        let mut walker = graph
            .neighbors_directed(node_index, Direction::Outgoing)
            .detach();

        while let Some((edge_idx, other_node_idx)) = walker.next(graph) {
            if let Some(other_state) = graph
                .node_weight(other_node_idx)
                .map(|node| node.get_state())
            {
                if matches!(other_state, NodeState::NotVisited) {
                    self.steps
                        .push_back(AlgorithmStep::Edge(EdgeStep::new(edge_idx)));
                    self.dfs_helper(graph, other_node_idx);
                }
            }
        }

        self.steps.push_back(AlgorithmStep::Node(NodeStep::new(
            node_index,
            NodeState::Visited,
        )));

        if let Some(node) = graph.node_weight_mut(node_index) {
            node.set_state(NodeState::Visited)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Dfs, GenericGraph};
    use crate::{
        graph::node::{Node, NodeState},
        step_algorithms::algorithm::{Algorithm, AlgorithmStep, EdgeStep, NodeStep},
    };
    use std::collections::VecDeque;

    #[derive(Clone)]
    struct SimpleNode {
        state: NodeState,
    }

    impl Node for SimpleNode {
        fn get_state(&self) -> &NodeState {
            &self.state
        }

        fn set_state(&mut self, state: NodeState) {
            self.state = state;
        }
    }

    #[test]
    fn small_test() {
        let mut graph = GenericGraph::<SimpleNode, u32>::new();
        let node_weight = SimpleNode {
            state: NodeState::NotVisited,
        };
        let a = graph.add_node(node_weight.clone());
        let b = graph.add_node(node_weight);
        let edge_idx = graph.add_edge(a, b, 0);

        let res = Dfs::new().run_algorithm(&mut graph, a);

        let mut desired = VecDeque::<AlgorithmStep>::new();
        desired.push_back(AlgorithmStep::Node(NodeStep::new(a, NodeState::Queued)));
        desired.push_back(AlgorithmStep::Edge(EdgeStep::new(edge_idx)));
        desired.push_back(AlgorithmStep::Node(NodeStep::new(b, NodeState::Queued)));
        desired.push_back(AlgorithmStep::Node(NodeStep::new(b, NodeState::Visited)));
        desired.push_back(AlgorithmStep::Node(NodeStep::new(a, NodeState::Visited)));

        assert_eq!(res.get_steps(), &desired);
    }
}
