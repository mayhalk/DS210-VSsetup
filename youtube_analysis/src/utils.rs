use crate::graph::Graph;
use std::fs::File;
use std::io::{self, BufRead};

pub fn load_graph_from_file(filename: &str) -> io::Result<Graph> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut graph = Graph::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let src = parts[0].parse::<u32>().unwrap();
        let dest = parts[1].parse::<u32>().unwrap();
        graph.add_edge(src, dest);
    }
    Ok(graph)
}
