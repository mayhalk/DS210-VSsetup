use std::collections::{HashMap, HashSet};

pub struct Graph {
    adj_list: HashMap<u32, Vec<u32>>, //graph of node and neighbors
}

impl Graph {
    pub fn new() -> Self {
        Self {
            adj_list: HashMap::new(), //initializes new empty graph
        }
    }

    pub fn add_edge(&mut self, src: u32, dest: u32) {
        self.adj_list.entry(src).or_default().push(dest);
        self.adj_list.entry(dest).or_default().push(src); //link both nodes
    } //undirected edge between source and destination

    pub fn get_neighbors(&self, vertex: &u32) -> Option<&Vec<u32>> {
        self.adj_list.get(vertex) //all of edges of a node
    }

    pub fn nodes(&self) -> HashSet<u32> {
        self.adj_list.keys().cloned().collect() //returns hashset of all nodes
    } 

    pub fn get_adj_list(&self) -> &HashMap<u32, Vec<u32>> {
        &self.adj_list //reference to entire adjacency list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_graph_is_empty() {
        let graph = Graph::new(); //initialize empty graph
        assert!(graph.adj_list.is_empty(), "New graph should be empty");
    }

    #[test]
    fn test_add_edge_creates_correct_links() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2); //checks correctly adds edge between nodes
        assert_eq!(graph.adj_list.len(), 2, "Graph should have two nodes after adding one edge.");
        assert!(graph.adj_list.get(&1).unwrap().contains(&2), "1 should link 2.");
        assert!(graph.adj_list.get(&2).unwrap().contains(&1), "2 should link 1.");
    }

    #[test]
    fn test_get_neighbors() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2); //check to correctly identify neighbors
        let neighbors_of_1 = graph.get_neighbors(&1).unwrap();
        assert!(neighbors_of_1.contains(&2), "Neighbors 1: should contain 2.");
        let neighbors_of_2 = graph.get_neighbors(&2).unwrap();
        assert!(neighbors_of_2.contains(&1), "Neighbors 2: should contain 1.");
    }

    #[test]
    fn test_nodes() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        let nodes = graph.nodes(); //checks if all nodes are listed
        assert_eq!(nodes.len(), 3, "Graph should contain three nodes");
        assert!(nodes.contains(&1), "Should contain 1");
        assert!(nodes.contains(&2), "Should contain 2");
        assert!(nodes.contains(&3), "Should contain 3");
    }

    #[test]
    fn test_get_adj_list() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2); 
        let adj_list = graph.get_adj_list(); //checks correctly represents graph
        assert!(adj_list.contains_key(&1) && adj_list.contains_key(&2), "Two nodes should exist");
        assert_eq!(adj_list[&1], vec![2], "1 should connect 2.");
        assert_eq!(adj_list[&2], vec![1], "2 should connect 1.");
    }
}

