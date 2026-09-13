mod graph_objects;

use crate::graph_objects::graph::Graph;

fn main() {
    let mut g = Graph::new(5, false);
    g.insert_edge(0, 1, 1.0);
}
