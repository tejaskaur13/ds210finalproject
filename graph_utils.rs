use petgraph::graph::UnGraph;

pub fn print_graph_info(graph: &UnGraph<(), ()>) {
    println!("Number of nodes: {}", graph.node_count());
    println!("Number of edges: {}", graph.edge_count());
}
