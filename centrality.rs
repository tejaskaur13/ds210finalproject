use petgraph::algo::{dijkstra, betweenness_centrality};
use petgraph::graph::UnGraph;

pub fn calculate_degree_centrality(graph: &UnGraph<(), ()>) -> Vec<f64> {
    let n = graph.node_count() as f64;
    graph.node_indices().map(|node| {
        graph.neighbors_undirected(node).count() as f64 / (n - 1.0)
    }).collect()
}

pub fn calculate_closeness_centrality(graph: &UnGraph<(), ()>) -> Vec<f64> {
    graph.node_indices().map(|node| {
        let path_lengths: Vec<_> = dijkstra(graph, node, None, |_| 1.0).values().collect();
        let total_path_length: f64 = path_lengths.iter().sum();
        if total_path_length > 0.0 {
            (graph.node_count() as f64 - 1.0) / total_path_length
        } else {
            0.0
        }
    }).collect()
}

pub fn calculate_betweenness_centrality(graph: &UnGraph<(), ()>) -> Vec<f64> {
    betweenness_centrality(graph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::UnGraph;

    #[test]
    fn test_calculate_degree_centrality() {
        let mut graph = UnGraph::new_undirected();
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        graph.add_edge(n1, n2, ());
        
        let centrality = calculate_degree_centrality(&graph);
        assert_eq!(centrality, vec![0.5, 0.5]);
    }

    #[test]
    fn test_calculate_closeness_centrality() {
        let mut graph = UnGraph::new_undirected();
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        let n3 = graph.add_node(());
        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());
        
        let centrality = calculate_closeness_centrality(&graph);
        let expected_values = vec![2.0 / 3.0, 1.0, 2.0 / 3.0];
        assert_eq!(centrality, expected_values);
    }

    #[test]
    fn test_calculate_betweenness_centrality() {
        let mut graph = UnGraph::new_undirected();
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        let n3 = graph.add_node(());
        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());
        
        let centrality = calculate_betweenness_centrality(&graph);
        let expected_values = vec![0.0, 1.0, 0.0];
        assert_eq!(centrality, expected_values);
    }
}
