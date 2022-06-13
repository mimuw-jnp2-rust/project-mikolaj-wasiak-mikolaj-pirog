use std::collections::VecDeque;

use petgraph::graph::NodeIndex;
use petgraph::Direction;

use crate::graph::node::{Node, NodeState};
use crate::step_algorithms::algorithm::GenericGraph;
use crate::step_algorithms::algorithm::{VisibleAlgorithm, Algorithm, AlgorithmStep, EdgeStep, NodeStep};
use crate::step_algorithms::timer::Timer;

pub struct Dfs {
    steps: VecDeque<AlgorithmStep>,
    timer: Timer,
    start_idx: NodeIndex,
    
}

impl<N: Node, E> Algorithm<N, E> for Dfs {
    fn start_idx(&self) -> NodeIndex {
        self.start_idx
    }

    fn steps(&self) -> &VecDeque<AlgorithmStep> {
        &self.steps
    }

    fn steps_mut(&mut self) -> &mut VecDeque<AlgorithmStep> {
        &mut self.steps
    }

    fn run_algorithm(&mut self, graph: &mut GenericGraph<N, E>) {
        Algorithm::reset_algorithm(self, graph);
        self.dfs(graph);
    }
}

impl VisibleAlgorithm for Dfs {
    fn timer(&self) -> &Timer {
        &self.timer
    }

    fn timer_mut(&mut self) -> &mut Timer {
        &mut self.timer
    }
}


impl Dfs {
    pub fn new(start_idx: NodeIndex) -> Dfs {
        Dfs {
            steps: VecDeque::new(),
            timer: Timer::new(1., true),
            start_idx,
        }
    }

    fn dfs<N: Node, E>(&mut self, graph: &mut GenericGraph<N, E>) {
        self.dfs_helper(graph, Algorithm::<N, E>::start_idx(self));
    }

    fn dfs_helper<N, E>(&mut self, graph: &mut GenericGraph<N, E>, node_index: NodeIndex) where N: Node, {
        Algorithm::<N, E>::add_step(self, AlgorithmStep::Node(NodeStep::new(
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
                    Algorithm::<N, E>::add_step(self,AlgorithmStep::Edge(EdgeStep::new(edge_idx)));
                    self.dfs_helper(graph, other_node_idx);
                }
            }
        }

        Algorithm::<N, E>::add_step(self, AlgorithmStep::Node(NodeStep::new(
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
    use crate::{graph::node::{NodeState, Node}, step_algorithms::algorithm::{Algorithm, AlgorithmStep, NodeStep, EdgeStep}};
    use super::{GenericGraph, Dfs};
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
        let node_weight = SimpleNode { state: NodeState::NotVisited };
        let a = graph.add_node(node_weight.clone());
        let b = graph.add_node(node_weight);
        let edge_idx = graph.add_edge(a, b, 0);

        let mut dfs = Dfs::new(a);
        dfs.run_algorithm(&mut graph);
        
        let mut desired = VecDeque::<AlgorithmStep>::new();
        desired.push_back(AlgorithmStep::Node(NodeStep::new(a, NodeState::Queued)));
        desired.push_back(AlgorithmStep::Edge(EdgeStep::new(edge_idx)));
        desired.push_back(AlgorithmStep::Node(NodeStep::new(b, NodeState::Queued)));
        desired.push_back(AlgorithmStep::Node(NodeStep::new(b, NodeState::Visited)));
        desired.push_back(AlgorithmStep::Node(NodeStep::new(a, NodeState::Visited)));

        assert_eq!(Algorithm::<SimpleNode, u32>::steps(&dfs), &desired);
    }
}