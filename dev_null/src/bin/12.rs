struct BinaryLifting {
    up: Vec<Vec<usize>>,
}

impl BinaryLifting {
    fn new(parent: &[usize]) -> Self {
        let n = parent.len() - 1;
        let levels = (usize::BITS - n.leading_zeros()).
    }
    }
}
