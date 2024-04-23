use crate::graph::Graph;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use flate2::read::GzDecoder; //Import GzDecoder

pub fn load_graph_from_file(filename: &str) -> io::Result<Graph> {
    let file = File::open(filename)?;
    let decoder = GzDecoder::new(file); //GzDecoder to decompress the file
    let reader = BufReader::new(decoder);
    let mut graph = Graph::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if !line.starts_with('#') && !line.trim().is_empty() {  // Skip comments and empty lines
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                let src = parts[0].parse::<u32>().unwrap();
                let dest = parts[1].parse::<u32>().unwrap();
                graph.add_edge(src, dest);
            }
        }
    }
    Ok(graph)
}





