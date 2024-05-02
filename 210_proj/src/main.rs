mod graph; //functions for graph structure
mod analysis; //bfs and more
mod utils; //reads data and creates graph

use std::fs::File;
use std::io::{Write, BufWriter};

fn main() {
    let filename = "data.txt"; //hardcode file

    let graph = utils::load_graph_from_file(filename).unwrap_or_else(|err| {
        eprintln!("Failed to load graph: {}", err); //create graph from data file
        std::process::exit(1);
    });

    //Setup the result file and write everything to it
    let result_file = File::create("results.txt").expect("Failed result file");
    let mut writer = BufWriter::new(result_file);

    //Connected components
    writeln!(writer, "Connected Components:").unwrap();
    writeln!(writer, "---------------------").unwrap();
    let components = analysis::find_connected_components(&graph);
    writeln!(writer, "Number of connected components: {}", components.len()).unwrap();
    for (i, component) in components.iter().enumerate() {
        writeln!(writer, "Component {}: Size {}", i + 1, component.len()).unwrap();
    }
    writeln!(writer, "").unwrap(); 

    //Degree Centrality
    writeln!(writer, "Degree Centrality Analysis:").unwrap();
    writeln!(writer, "---------------------------").unwrap();
    let centrality = analysis::degree_centrality(&graph);
    let (highest_degree_node, highest_degree) = analysis::highest_degree_node(&centrality).expect("Highest degree error");
    writeln!(writer, "Highest degree centrality: Node {} with a degree of {}", highest_degree_node, highest_degree).unwrap();
    writeln!(writer, "").unwrap();

    //Strategic start vertex
    let start_vertex = highest_degree_node;

    //Average path length
    writeln!(writer, "Average Path Length Analysis:").unwrap();
    writeln!(writer, "--------------------").unwrap();
    let avg_path_length = analysis::calculate_average_path_length(&graph);
    writeln!(writer, "Average path length: {:.2}", avg_path_length).unwrap();
    writeln!(writer, "").unwrap(); 

    //BFS Analysis
    let bfs_start_vertex = 1; //start vertex for bfs node 1
    writeln!(writer, "BFS Analysis:").unwrap();
    writeln!(writer, "-------------").unwrap();
    let visited = analysis::bfs(&graph, bfs_start_vertex);
    writeln!(writer, "Visited nodes from node {}: ", bfs_start_vertex).unwrap();
    for node in visited.iter() {
        writeln!(writer, "- Node {}", node).unwrap();
    }
    writeln!(writer, "").unwrap(); 

    //Shortest Paths
    writeln!(writer, "Shortest Paths from node {}: ", start_vertex).unwrap();
    writeln!(writer, "----------------------------").unwrap();
    let shortest_paths = analysis::bfs_shortest_paths(&graph, start_vertex);
    for (node, distance) in shortest_paths.iter() {
        writeln!(writer, "- Node {}: {} steps", node, distance).unwrap();
    }
    writeln!(writer, "").unwrap(); 

    println!("Analysis Complete"); //extra completion identifier
}