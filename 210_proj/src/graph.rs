use std::collections::{HashMap, HashSet};

pub struct Graph {
    adj_list: HashMap<u32, Vec<u32>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, src: u32, dest: u32) {
        self.adj_list.entry(src).or_default().push(dest);
        self.adj_list.entry(dest).or_default().push(src);
    }

    pub fn get_neighbors(&self, vertex: &u32) -> Option<&Vec<u32>> {
        self.adj_list.get(vertex)
    }

    pub fn nodes(&self) -> HashSet<u32> {
        self.adj_list.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_graph_is_empty() {
        let graph = Graph::new();
        assert!(graph.adj_list.is_empty(), "New graph should be empty");
    }

    #[test]
    fn test_add_edge_creates_correct_links() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        assert_eq!(graph.adj_list.len(), 2, "Graph should have two nodes after adding one edge.");
        assert!(graph.adj_list.get(&1).unwrap().contains(&2), "Node 1 should be linked to Node 2.");
        assert!(graph.adj_list.get(&2).unwrap().contains(&1), "Node 2 should be linked to Node 1.");
    }

    #[test]
    fn test_get_neighbors() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        let neighbors_of_1 = graph.get_neighbors(&1).unwrap();
        assert!(neighbors_of_1.contains(&2), "Neighbors of node 1 should include node 2.");
        let neighbors_of_2 = graph.get_neighbors(&2).unwrap();
        assert!(neighbors_of_2.contains(&1), "Neighbors of node 2 should include node 1.");
    }

    #[test]
    fn test_nodes() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        let nodes = graph.nodes();
        assert_eq!(nodes.len(), 3, "Graph should contain three nodes");
        assert!(nodes.contains(&1), "Nodes should include node 1");
        assert!(nodes.contains(&2), "Nodes should include node 2");
        assert!(nodes.contains(&3), "Nodes should include node 3");
    }
}

