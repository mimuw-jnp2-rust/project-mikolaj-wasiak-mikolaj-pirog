use petgraph::Graph;

fn main() {
    let mut graph = Graph::<i32,i32>::new();
    let index = graph.add_node(23);
    let index2 = graph.add_node(23);
    graph.add_edge(index, index, 0);
    graph.add_edge(index, index2, 0);
}
