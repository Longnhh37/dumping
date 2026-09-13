use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct Edge<W> {
    from: u32,
    to: u32,
    pub weight: W,
}

impl<W> Edge<W> {
    pub fn new(from: u32, to: u32, weight: W) -> Self {
        Self { from, to, weight }
    }

    pub fn from(&self) -> u32 {
        self.from
    }

    pub fn to(&self) -> u32 {
        self.to
    }
}

impl<W: PartialEq> PartialEq for Edge<W> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl<W: PartialEq> Eq for Edge<W> {}

impl<W: PartialOrd> PartialOrd for Edge<W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<W: PartialOrd> Ord for Edge<W> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("weight cannot be compared with NaN")
    }
}
