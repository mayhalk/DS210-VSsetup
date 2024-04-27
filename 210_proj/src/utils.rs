use crate::graph::Graph;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn load_graph_from_file(filename: &str) -> io::Result<Graph> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut graph = Graph::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if line.starts_with('#') || line.trim().is_empty() { //skip comments and empty lines
            continue; 
        }
        let mut parts = line.split_whitespace();
        if let (Some(src_str), Some(dest_str)) = (parts.next(), parts.next()) {
            let src = match src_str.parse::<u32>() {
                Ok(num) => num,
                Err(_) => continue, //error handling
            };
            let dest = match dest_str.parse::<u32>() {
                Ok(num) => num,
                Err(_) => continue, //error handling
            };
            graph.add_edge(src, dest);
        }
    }

    Ok(graph)
}







