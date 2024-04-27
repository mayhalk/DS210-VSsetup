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


