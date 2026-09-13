#[derive(Debug, Clone)]
pub struct Edge {
    from: u32,
    to: u32,
    pub weight: f64,
}

impl Edge {
    pub fn new(from: u32, to: u32, weight: f64) -> Self {
        Self { from, to, weight }
    }
}
