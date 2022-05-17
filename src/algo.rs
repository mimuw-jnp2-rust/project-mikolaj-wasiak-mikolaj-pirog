use std::collections::VecDeque;

use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::Direction;
use tetra::Context;

use crate::graph::node::NodeState;
use crate::graph::Graph;

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
    idx: EdgeIndex,
}

pub enum AlgorithmStep {
    Node(NodeStep),
    Edge(EdgeStep),
}

pub struct Timer {
    time_remaining: f32,
    active: bool,
    loops: bool,
    time: f32,
}

impl Timer {
    pub fn new(time: f32, loops: bool) -> Timer {
        Timer {
            time,
            active: false,
            loops,
            time_remaining: 0.,
        }
    }

    pub fn start(&mut self) {
        self.active = true;
        self.time_remaining = self.time;
    }

    pub fn stop(&mut self) {
        self.active = false;
        self.time_remaining = 0.;
    }

    fn finished(&mut self) -> bool {
        if self.active && self.time_remaining <= 0. {
            if self.loops {
                self.start();
            } else {
                self.stop();
            }
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, ctx: &mut Context) -> bool {
        if self.active {
            self.time_remaining -= tetra::time::get_delta_time(ctx).as_secs_f32();
            self.finished()
        } else {
            false
        }
    }
}

pub struct Dfs {
    steps: VecDeque<AlgorithmStep>,
    timer: Timer,
    start_idx: NodeIndex,
}

impl Dfs {
    fn dfs_helper(&mut self, graph: &mut Graph, node_index: NodeIndex) {
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
                        .push_back(AlgorithmStep::Edge(EdgeStep { idx: edge_idx }));
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

    pub fn dfs(&mut self, graph: &mut Graph, node_index: NodeIndex) {
        self.dfs_helper(graph, node_index);
    }

    pub fn new() -> Dfs {
        Dfs {
            steps: VecDeque::new(),
            timer: Timer::new(0.5, true),
            start_idx: NodeIndex::new(0),
        }
    }
}

pub trait ShowAlgorithm {
    fn run_algorithm(&mut self, graph: &mut Graph, starting_node_idx: NodeIndex);
    fn update(&mut self, ctx: &mut Context, graph: &mut Graph);
}

impl ShowAlgorithm for Dfs {
    fn run_algorithm(&mut self, graph: &mut Graph, starting_node_idx: NodeIndex) {
        self.start_idx = starting_node_idx;
        for node in graph.node_weights_mut() {
            node.set_state(NodeState::NotVisited);
        }

        self.dfs(graph, self.start_idx);
        for node in graph.node_weights_mut() {
            node.set_state(NodeState::NotVisited);
        }
        for edge in graph.edge_weights_mut() {
            edge.enable_edge();
            edge.disable_edge();
        }

        // TODO: To do oddzielnej funkcji?
        self.timer.start();
        graph
            .node_weight_mut(self.start_idx)
            .map(|node| node.set_ignore_force(true));
    }

    fn update(&mut self, ctx: &mut Context, graph: &mut Graph) {
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
                graph
                    .node_weight_mut(self.start_idx)
                    .map(|node| node.set_ignore_force(false));
            }
        }
    }
}
