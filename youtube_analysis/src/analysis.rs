use crate::graph::Graph;
use std::collections::{HashSet, VecDeque};

pub fn bfs(graph: &Graph, start_vertex: u32) -> HashSet<u32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start_vertex);
    queue.push_back(start_vertex);

    while let Some(v) = queue.pop_front() {
        if let Some(neighbors) = graph.get_neighbors(&v) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    visited
}


