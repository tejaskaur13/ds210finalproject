// Importing necessary modules and libraries
mod graph_utils;
mod centrality;
mod parser;

use petgraph::graph::UnGraph;
use petgraph::algo::connected_components;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Reading and parsing the graph from a CSV file
    let graph = parser::read_and_parse_graph("data/econ-beacxc.csv")?;
    
    // Displaying basic information about the graph
    graph_utils::print_graph_info(&graph);

    // Calculating various centrality measures
    let degree_centrality = centrality::calculate_degree_centrality(&graph);
    let closeness_centrality = centrality::calculate_closeness_centrality(&graph);
    let betweenness_centrality = centrality::calculate_betweenness_centrality(&graph);

    // Displaying centrality measures
    println!("Degree Centrality: {:?}", degree_centrality);
    println!("Closeness Centrality: {:?}", closeness_centrality);
    println!("Betweenness Centrality: {:?}", betweenness_centrality);

    Ok(())
}
