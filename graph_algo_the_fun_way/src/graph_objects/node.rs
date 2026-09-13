use std::collections::HashMap;

use super::edge::Edge;

pub struct Node {
    idx: u32,
    edges: HashMap<u32, Edge>, // destination index -> Edge
    label: Option<i32>,
}

impl Node {
    pub fn new(idx: u32, label: Option<i32>) -> Self {
        Self {
            idx,
            edges: HashMap::new(),
            label,
        }
    }

    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn get_edge(&self, neighbor: u32) -> Option<&Edge> {
        self.edges.get(&neighbor)
    }

    pub fn add_edge(&mut self, neighbor: u32, weight: f64) {
        let new_edge = Edge::new(self.idx, neighbor, weight);
        self.edges.insert(self.idx, new_edge);
    }

    pub fn remove_edge(&mut self, neighbor: u32) {
        self.edges.remove(&neighbor);
    }

    pub fn get_edge_list(&self) -> Vec<&Edge> {
        self.edges.values().collect()
    }

    pub fn get_sorted_edge_list(&self) -> Vec<&Edge> {
        let mut res: Vec<&Edge> = self.edges.values().collect();
        res.sort_unstable_by_key(|&e| e.weight);
        res
    }
}
