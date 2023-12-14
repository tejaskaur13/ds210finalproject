use petgraph::graph::{UnGraph, NodeIndex};
use csv::ReaderBuilder;
use std::error::Error;
use std::path::Path;

pub fn read_and_parse_graph<P: AsRef<Path>>(file_path: P) -> Result<UnGraph<(), ()>, Box<dyn Error>> {
    // Function to read and parse graph from CSV
    let mut reader = ReaderBuilder::new().from_path(file_path)?;
    let mut graph = UnGraph::<(), ()>::new_undirected();
    let mut node_indices = Vec::new();

    for result in reader.records() {
        let record = result?;
        let from: usize = record[0].parse()?;
        let to: usize = record[1].parse()?;

        while node_indices.len() <= from || node_indices.len() <= to {
            node_indices.push(graph.add_node(()));
        }

        graph.add_edge(node_indices[from], node_indices[to], ());
    }

    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_read_and_parse_graph() -> Result<(), Box<dyn Error>> {
        // Creating a temporary CSV file for testing
        let mut file = File::create("test_graph.csv")?;
        writeln!(file, "0,1\n1,2")?;

        let graph = read_and_parse_graph("test_graph.csv")?;
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
        Ok(())
    }
}
