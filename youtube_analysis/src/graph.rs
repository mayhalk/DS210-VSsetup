use std::collections::HashMap;

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
        self.adj_list.entry(src).or_insert(vec![]).push(dest);
        self.adj_list.entry(dest).or_insert(vec![]).push(src); //undirected
    }

    pub fn get_neighbors(&self, vertex: &u32) -> Option<&Vec<u32>> {
        self.adj_list.get(vertex)
    }
}

