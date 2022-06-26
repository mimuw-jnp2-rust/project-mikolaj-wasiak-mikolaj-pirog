use std::collections::{HashMap, VecDeque};

use dyn_partial_eq::DynPartialEq;
use petgraph::{graph::NodeIndex, Graph};
use tetra::graphics::Color;

use crate::graph::GraphOnCanvas;

use super::{
    dfs, step_algorithm::Step, Dfs, DirectedStepAlgorithm, StepAlgorithm, StepAlgorithmResult,
};

// Found them as rgb8 but rgb8 function isn't const
static COLORS: [Color; 10] = [
    Color::rgb(1., 95. / 255., 93. / 255.),
    Color::rgb(0., 204. / 255., 191. / 255.),
    Color::rgb(114. / 255., 242. / 255., 235. / 255.),
    Color::rgb(116. / 255., 126. / 255., 126. / 255.),
    Color::rgb(63. / 255., 124. / 255., 133. / 255.),
    Color::rgb(239. / 255., 96. / 255., 36. / 255.),
    Color::rgb(240. / 255., 148. / 255., 31. / 255.),
    Color::rgb(144. / 255., 161. / 255., 157. / 255.),
    Color::rgb(25. / 255., 103. / 255., 116. / 255.),
    Color::rgb(54. / 255., 52. / 255., 50. / 255.),
];

#[derive(DynPartialEq, PartialEq, Debug)]
pub struct Reverse {}

impl Step for Reverse {
    fn apply_step(&self, graph: &mut crate::graph::Graph) {
        graph.edge_weights_mut().for_each(|edge| edge.reverse());
    }
}

#[derive(DynPartialEq, PartialEq, Debug)]
pub struct PaintComponent {
    color: Color,
    indices: Vec<NodeIndex>,
}

impl Step for PaintComponent {
    fn apply_step(&self, graph: &mut crate::graph::Graph) {
        self.indices.iter().for_each(|idx| {
            if let Some(node) = graph.node_weight_mut(*idx) {
                node.set_color(self.color);
            }
        });
    }
}

#[derive(DynPartialEq, PartialEq, Debug)]
pub struct ResetState {}

impl Step for ResetState {
    fn apply_step(&self, graph: &mut crate::graph::Graph) {
        graph.reset_state();
        graph.edge_weights_mut().for_each(|edge| edge.disable());
    }
}

#[derive(DynPartialEq, PartialEq, Debug)]
pub struct EnableEdges {}

impl Step for EnableEdges {
    fn apply_step(&self, graph: &mut crate::graph::Graph) {
        graph.edge_weights_mut().for_each(|edge| edge.enable());
    }
}

pub struct Scc {
    steps: VecDeque<Box<dyn Step>>,
    components: HashMap<usize, Vec<NodeIndex>>,
}

impl DirectedStepAlgorithm for Scc {
    fn run<N, E>(&mut self, graph: &Graph<N, E>, _start_idx: NodeIndex) {
        let dfs = self.postorder_dfs(graph);
        let rev_dfs = self.reversed_dfs(graph, dfs.postorder());
        self.steps = dfs.into_steps();
        self.steps.push_back(Box::new(ResetState {}));
        self.steps.push_back(Box::new(Reverse {}));
        let mut rev_steps = rev_dfs.into_steps();
        self.steps.append(&mut rev_steps);

        // Visual cleanup
        self.steps.push_back(Box::new(Reverse {}));
        self.steps.push_back(Box::new(EnableEdges {}));
    }

    fn result(self) -> super::StepAlgorithmResult {
        StepAlgorithmResult::from_steps(self.steps)
    }
}

impl Scc {
    pub fn new() -> Scc {
        Scc {
            steps: VecDeque::new(),
            components: HashMap::new(),
        }
    }

    fn postorder_dfs<N, E>(&mut self, graph: &Graph<N, E>) -> Dfs {
        let mut dfs = Dfs::from_graph(graph);
        graph.node_indices().for_each(|idx| {
            if let Some(state) = dfs.states().get(&idx) {
                if matches!(state, dfs::NodeState::NotVisited) {
                    dfs.run(graph, idx);
                }
            }
        });
        dfs
    }

    fn reversed_dfs<N, E>(&mut self, graph: &Graph<N, E>, order: &[NodeIndex]) -> Dfs {
        let mut dfs = Dfs::from_graph(graph);
        let mut nr = 0;
        order.iter().rev().for_each(|idx| {
            if let Some(state) = dfs.states().get(idx) {
                if matches!(state, dfs::NodeState::NotVisited) {
                    dfs.dfs_reversed(graph, *idx);
                    self.components.insert(nr, dfs.postorder().clone());
                    let paint_step = PaintComponent {
                        color: COLORS[nr % 10],
                        indices: dfs.postorder().clone(),
                    };
                    dfs.postorder_mut().clear();
                    dfs.steps_mut().push_back(Box::new(paint_step));
                    nr += 1;
                }
            }
        });
        dfs
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::step_algorithms::DirectedStepAlgorithm;

    use super::Scc;

    #[test]
    fn two_triangles() {
        let mut graph = petgraph::Graph::<u32, u32, petgraph::Directed>::new();
        let a = graph.add_node(0);
        let b = graph.add_node(0);
        let c = graph.add_node(0);
        let d = graph.add_node(0);
        let e = graph.add_node(0);
        let f = graph.add_node(0);

        graph.add_edge(a, b, 0);
        graph.add_edge(b, c, 0);
        graph.add_edge(c, a, 0);
        graph.add_edge(c, d, 0);
        graph.add_edge(d, e, 0);
        graph.add_edge(e, f, 0);
        graph.add_edge(f, d, 0);

        let mut scc = Scc::new();

        // a doesn't matter here, it runs from random node
        scc.run(&graph, a);

        assert!(scc.components.contains_key(&0_usize));
        assert!(scc.components.contains_key(&1_usize));
        assert_eq!(scc.components.len(), 2);

        // Change vector to hashset for set comparation
        assert_eq!(
            HashSet::from_iter(scc.components.remove(&0_usize).unwrap().iter()),
            HashSet::from([&a, &b, &c])
        );
        assert_eq!(
            HashSet::from_iter(scc.components.remove(&1_usize).unwrap().iter()),
            HashSet::from([&d, &e, &f])
        );
    }
}
