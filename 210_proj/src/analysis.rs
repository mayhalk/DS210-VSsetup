use crate::graph::Graph;
use std::collections::{HashSet, VecDeque, HashMap};

//find neighbors of a node
fn process_neighbors(node: u32, graph: &Graph, visited: &mut HashSet<u32>, queue: &mut VecDeque<u32>) {
    if let Some(neighbors) = graph.get_neighbors(&node) {
        for &neighbor in neighbors {
            if visited.insert(neighbor) {  //check and insert
                queue.push_back(neighbor);
            }
        }
    }
}

pub fn bfs(graph: &Graph, start_vertex: u32) -> HashSet<u32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start_vertex);
    queue.push_back(start_vertex);

    while let Some(v) = queue.pop_front() {
        process_neighbors(v, graph, &mut visited, &mut queue);
    }

    visited
}

pub fn bfs_shortest_paths(graph: &Graph, start: u32) -> HashMap<u32, u32> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    distances.insert(start, 0);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let current_distance = distances[&node];

        if let Some(neighbors) = graph.get_neighbors(&node) {
            for &neighbor in neighbors {
                distances.entry(neighbor).or_insert_with(|| {
                    queue.push_back(neighbor);
                    current_distance + 1
                });
            }
        }
    }

    distances
}

pub fn calculate_average_path_length(graph: &Graph) -> f64 {
    let total_distance: u64 = graph.nodes().iter().map(|&node| {
        bfs_shortest_paths(graph, node).values().map(|&d| u64::from(d)).sum::<u64>()
    }).sum();

    let paths_count: u64 = graph.nodes().iter().map(|&node| {
        bfs_shortest_paths(graph, node).len() as u64
    }).sum();

    if paths_count > 0 {
        total_distance as f64 / paths_count as f64
    } else {
        0.0
    }
}

pub fn find_connected_components(graph: &Graph) -> Vec<Vec<u32>> {
    let mut visited = HashSet::new();
    let mut components = Vec::new();

    for node in graph.nodes() {
        if !visited.contains(&node) {
            components.push(collect_component(graph, node, &mut visited));
        }
    }

    components
}

fn collect_component(graph: &Graph, start: u32, visited: &mut HashSet<u32>) -> Vec<u32> {
    let mut queue = VecDeque::new();
    let mut component = Vec::new();

    visited.insert(start);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        component.push(node);
        process_neighbors(node, graph, visited, &mut queue);
    }

    component
}











