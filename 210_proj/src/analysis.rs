use crate::graph::Graph;
use std::collections::{HashMap, HashSet, VecDeque};
use rand::prelude::*;

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
        let current_distance = *distances.get(&node).unwrap();

        if let Some(neighbors) = graph.get_neighbors(&node) {
            for &neighbor in neighbors {
                if !distances.contains_key(&neighbor) {
                    queue.push_back(neighbor);
                    distances.insert(neighbor, current_distance + 1);
                }
            }
        }
    }

    distances
}

pub fn calculate_average_path_length(graph: &Graph) -> f64 {
    let mut rng = thread_rng();
    let nodes: Vec<u32> = graph.nodes().into_iter().collect();
    let sampled_nodes = nodes.choose_multiple(&mut rng, 1000).cloned().collect::<Vec<u32>>();

    let mut total_distance: u64 = 0;
    let mut paths_count: u64 = 0;

    for &node in &sampled_nodes {
        let shortest_paths = bfs_shortest_paths(graph, node);
        for &target in &sampled_nodes {
            if let Some(&distance) = shortest_paths.get(&target) {
                total_distance += distance as u64;
                paths_count += 1;
            }
        }
    }

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
