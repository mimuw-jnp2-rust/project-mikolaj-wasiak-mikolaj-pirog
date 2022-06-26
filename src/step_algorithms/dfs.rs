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
            edge.enable();
        }
    }
}

pub struct Dfs {
    steps: VecDeque<Box<dyn Step>>,
    states: HashMap<NodeIndex, NodeState>,
    preorder: Vec<NodeIndex>,
    postorder: Vec<NodeIndex>,
}

impl StepAlgorithm for Dfs {
    fn run<N, E, D: EdgeType>(&mut self, graph: &Graph<N, E, D>, start_idx: NodeIndex) {
        self.dfs(graph, start_idx);
    }

    fn result(self) -> StepAlgorithmResult {
        StepAlgorithmResult::from_steps(self.into_steps())
    }
}

impl Dfs {
    pub fn states(&self) -> &HashMap<NodeIndex, NodeState> {
        &self.states
    }

    pub fn _preorder(&self) -> &Vec<NodeIndex> {
        &self.preorder
    }

    pub fn postorder(&self) -> &Vec<NodeIndex> {
        &self.postorder
    }

    pub fn postorder_mut(&mut self) -> &mut Vec<NodeIndex> {
        &mut self.postorder
    }

    pub fn into_steps(self) -> VecDeque<Box<dyn Step>> {
        self.steps
    }

    pub fn steps_mut(&mut self) -> &mut VecDeque<Box<dyn Step>> {
        &mut self.steps
    }

    pub fn from_graph<N, E, D: EdgeType>(graph: &Graph<N, E, D>) -> Dfs {
        let mut states = HashMap::new();
        for index in graph.node_indices() {
            states.insert(index, NodeState::NotVisited);
        }
        Dfs {
            steps: VecDeque::new(),
            states,
            preorder: Vec::new(),
            postorder: Vec::new(),
        }
    }

    fn dfs<N, E, D: EdgeType>(&mut self, graph: &Graph<N, E, D>, node_index: NodeIndex) {
        self.dfs_helper(graph, node_index, Direction::Outgoing);
    }

    pub fn dfs_reversed<N, E, D: EdgeType>(
        &mut self,
        graph: &Graph<N, E, D>,
        node_index: NodeIndex,
    ) {
        self.dfs_helper(graph, node_index, Direction::Incoming);
    }

    fn dfs_helper<N, E, D: EdgeType>(
        &mut self,
        graph: &Graph<N, E, D>,
        node_index: NodeIndex,
        direction: Direction,
    ) {
        self.steps
            .push_back(Box::new(NodeStep::new(node_index, NodeState::Queued)));

        self.states.insert(node_index, NodeState::Queued);
        self.preorder.push(node_index);

        let mut walker = graph.neighbors_directed(node_index, direction).detach();

        while let Some((edge_idx, other_node_idx)) = walker.next(graph) {
            if let Some(other_state) = self.states.get(&other_node_idx) {
                if matches!(other_state, NodeState::NotVisited) {
                    self.steps.push_back(Box::new(EdgeStep::new(edge_idx)));
                    self.dfs_helper(graph, other_node_idx, direction);
                }
            }
        }

        self.steps
            .push_back(Box::new(NodeStep::new(node_index, NodeState::Visited)));

        self.states.insert(node_index, NodeState::Visited);
        self.postorder.push(node_index);
    }
}

#[cfg(test)]
mod tests {
    use petgraph::EdgeType;

    use super::Dfs;
    use crate::step_algorithms::{
        dfs::{EdgeStep, NodeState, NodeStep},
        step_algorithm::Step,
        StepAlgorithm,
    };
    use std::collections::VecDeque;

    fn small_test_main<N: Default, E: Default, D: EdgeType>(mut graph: petgraph::Graph<N, E, D>) {
        let a = graph.add_node(N::default());
        let b = graph.add_node(N::default());
        let edge_idx = graph.add_edge(a, b, E::default());

        let mut dfs = Dfs::from_graph(&graph);
        dfs.run(&graph, a);

        let res = dfs.result();

        let mut desired = VecDeque::<Box<dyn Step>>::new();
        desired.push_back(Box::new(NodeStep::new(a, NodeState::Queued)));
        desired.push_back(Box::new(EdgeStep::new(edge_idx)));
        desired.push_back(Box::new(NodeStep::new(b, NodeState::Queued)));
        desired.push_back(Box::new(NodeStep::new(b, NodeState::Visited)));
        desired.push_back(Box::new(NodeStep::new(a, NodeState::Visited)));

        assert_eq!(res.steps(), &desired);
    }

    #[test]
    fn small_test_directed() {
        let graph = petgraph::Graph::<u32, u32, petgraph::Directed>::new();
        small_test_main(graph);
    }

    #[test]
    fn small_test_undirected() {
        let graph = petgraph::Graph::<u32, u32, petgraph::Undirected>::new_undirected();
        small_test_main(graph);
    }
}
