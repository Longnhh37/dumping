use super::{edge::Edge, node::Node};

pub struct Graph<W> {
    nodes: Vec<Node<W>>,
    num_nodes: u32,
    undirected: bool,
}

impl<W: Copy + PartialOrd> Graph<W> {
    pub fn new(num_nodes: u32, undirected: bool) -> Self {
        let nodes = (0..num_nodes).map(|idx| Node::new(idx, None)).collect();

        Self {
            nodes,
            num_nodes,
            undirected,
        }
    }

    pub fn get_edge(&self, from: i32, to: i32) -> Option<&Edge<W>> {
        if !self.valid_input(from, to) {
            return None;
        }
        self.nodes[from as usize].get_edge(to as u32)
    }

    pub fn is_edge(&self, from: i32, to: i32) -> bool {
        self.get_edge(from, to).is_some()
    }

    pub fn make_edge_list(&self) -> Vec<&Edge<W>> {
        self.nodes.iter().flat_map(|n| n.get_edge_list()).collect()
    }

    pub fn insert_edge(&mut self, from: i32, to: i32, weight: W) {
        if !self.valid_input(from, to) {
            return;
        }

        self.nodes[from as usize].add_edge(to as u32, weight);
        if self.undirected {
            self.nodes[to as usize].add_edge(from as u32, weight);
        }
    }

    pub fn remove_edge(&mut self, from: i32, to: i32) {
        if !self.valid_input(from, to) {
            return;
        }
        self.nodes[from as usize].remove_edge(to as u32);
        if self.undirected {
            self.nodes[to as usize].remove_edge(from as u32);
        }
    }

    pub fn copy_graph(&self) -> Self {
        let nodes = self.nodes.iter().map(Node::deep_copy).collect();

        Self {
            nodes,
            num_nodes: self.num_nodes,
            undirected: self.undirected,
        }
    }

    // ================================================================
    // helpers
    // ================================================================
    fn valid_input(&self, from: i32, to: i32) -> bool {
        if from < 0 || from >= self.num_nodes as i32 || to < 0 || to >= self.num_nodes as i32 {
            return false;
        }
        true
    }
}
