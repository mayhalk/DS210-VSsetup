mod graph; //functions for graph structure
mod analysis; //bfs and more
mod utils; //reads data and creates graph

fn main() {
    let filename = "data.txt"; 
    let start_vertex = 1; 

    let graph = utils::load_graph_from_file(filename).unwrap_or_else(|err| {
        eprintln!("Failed to load graph: {}", err);
        std::process::exit(1);
    });

    //BFS
    let visited = analysis::bfs(&graph, start_vertex);
    println!("Visited nodes from {}: {:?}", start_vertex, visited);

    //Connected components
    let components = analysis::find_connected_components(&graph);
    println!("Number of connected components: {}", components.len());
    for (i, component) in components.iter().enumerate() {
        println!("Component {}: Size {}", i + 1, component.len());
    }

    //Average path length
    let avg_path_length = analysis::calculate_average_path_length(&graph);
    println!("Average path length: {:.2}", avg_path_length);

    //Shortest paths
    let shortest_paths = analysis::bfs_shortest_paths(&graph, start_vertex);
    println!("Shortest paths from node {}: {:?}", start_vertex, shortest_paths);
}




