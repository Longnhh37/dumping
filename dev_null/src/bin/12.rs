struct BinaryLifting {
    up: Vec<Vec<usize>>,
}

impl BinaryLifting {
    fn new(parent: &[usize]) -> Self {
        let n = parent.len() - 1;
        let levels = (usize::BITS - n.leading_zeros()).max(1) as usize;

        let mut up = vec![vec![0; n + 1]; levels];
        up[0].copy_from_slice(parent);
        up[0][0] = 0;

        for j in 1..levels {
            for x in 1..=n {
                let mid = up[j - 1][x];
                up[j][x] = up[j - 1][mid];
            }
        }

        Self { up }
    }

    fn ancestor(&self, mut x: usize, k: usize) -> usize {
        if k >> self.up.len() != 0 {
            return 0;
        }
        for j in 0..self.up.len() {
            if k >> j & 1 == 1 {
                x = self.up[j][x];
                if x == 0 {
                    break;
                }
            }
        }

        x
    }
}

fn main() {
    //        x:  0  1  2  3  4  5  6  7  8
    let parent = [0, 0, 1, 4, 1, 1, 2, 4, 7];
    let bl = BinaryLifting::new(&parent);

    assert_eq!(bl.ancestor(8, 1), 7);
    assert_eq!(bl.ancestor(8, 2), 4);
    assert_eq!(bl.ancestor(8, 3), 1);
    assert_eq!(bl.ancestor(8, 4), 0);
    println!("ok");
}
