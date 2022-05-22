use petgraph::Direction;
use petgraph::graph::NodeIndex;

use crate::graph::Graph;
use crate::graph::node::NodeState;
use crate::step_algorithms::algorithm::{Algorithm, AlgorithmStep, EdgeStep, NodeStep};

impl Algorithm {
    fn dfs(&mut self, graph: &mut Graph) {
        self.dfs_helper(graph, self.start_idx());
    }

    fn dfs_helper(&mut self, graph: &mut Graph, node_index: NodeIndex) {
        self.add_step(AlgorithmStep::Node(NodeStep::new(
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
                    self.add_step(AlgorithmStep::Edge(EdgeStep { idx: edge_idx }));
                    self.dfs_helper(graph, other_node_idx);
                }
            }
        }

        self.add_step(AlgorithmStep::Node(NodeStep::new(
            node_index,
            NodeState::Visited,
        )));

        if let Some(node) = graph.node_weight_mut(node_index) {
            node.set_state(NodeState::Visited)
        }
    }

    pub fn show_dfs(&mut self, graph: &mut Graph) {
        for node in graph.node_weights_mut() {
            node.set_state(NodeState::NotVisited);
        }

        self.dfs(graph);

        for node in graph.node_weights_mut() {
            node.set_state(NodeState::NotVisited);
        }

        // those lines allow node to move while the algorithm is being showcased.
        for edge in graph.edge_weights_mut() {
            edge.enable_edge();
            edge.disable_edge();
        }

        self.start_timer();
        self.turn_off_start_node_gravity(graph);
    }
}
