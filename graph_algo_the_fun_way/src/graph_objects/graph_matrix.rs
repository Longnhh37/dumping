struct GraphMatrix {
    num_nodes: usize,
    undirected: bool,
    connections: Vec<Vec<f32>>,
}

impl GraphMatrix {
    fn new(n: usize, undirected: bool) -> Self {
        Self {
            num_nodes: n,
            undirected,
            connections: vec![vec![0.0; n]; n],
        }
    }
}
