use std::collections::HashMap;

pub struct Graph {
    adj_list: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list.entry(u).or_insert(Vec::new()).push(v);
        self.adj_list.entry(v).or_insert(Vec::new()).push(u); //undirected graph
    }
}
