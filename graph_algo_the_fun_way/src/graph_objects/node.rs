use std::collections::HashMap;

use super::edge::Edge;

pub struct Node<W> {
    idx: u32,
    edges: HashMap<u32, Edge<W>>, // destination index -> Edge
    label: Option<i32>,
}

impl<W: Copy + PartialOrd> Node<W> {
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

    pub fn get_edge(&self, neighbor: u32) -> Option<&Edge<W>> {
        self.edges.get(&neighbor)
    }

    pub fn add_edge(&mut self, neighbor: u32, weight: W) {
        let new_edge = Edge::new(self.idx, neighbor, weight);
        self.edges.insert(neighbor, new_edge);
    }

    pub fn remove_edge(&mut self, neighbor: u32) {
        self.edges.remove(&neighbor);
    }

    pub fn get_edge_list(&self) -> Vec<&Edge<W>> {
        self.edges.values().collect()
    }

    pub fn get_sorted_edge_list(&self) -> Vec<&Edge<W>> {
        let mut res: Vec<&Edge<W>> = self.edges.values().collect();
        res.sort_unstable();
        res
    }

    pub fn deep_copy(&self) -> Self {
        let edges = self.edges.iter().map(|(&k, &e)| (k, e)).collect();
        Self {
            idx: self.idx,
            edges,
            label: self.label,
        }
    }
}
