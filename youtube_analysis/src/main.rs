mod graph; //functions for graph structure
mod analysis; //bfs and more
mod utils; //reads data and creates graph
fn main() {
        let filename = "data.txt.gz";
        let graph = utils::load_graph_from_file(filename).expect("Graph failed");
    
        //bfs
        let visited = analysis::bfs(&graph, 1);  //node 1
        println!("Visited nodes: {:?}", visited);
}
