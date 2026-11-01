// Generic associated types (GATs)
// lending iterators - Rust 1.65+

// Without GATs — impossible to express a lending iterator:
// trait LendingIterator {
//     type Item<'a>;  // ← This was rejected before 1.65
// }

// With GATs (Rust 1.65+):
trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

struct WindowIter<'data> {
    data: &'data [u8],
    pos: usize,
    window_size: usize,
}

impl<'data> LendingIterator for WindowIter<'data> {
    type Item<'a>
        = &'a [u8]
    where
        Self: 'a;

    fn next(&mut self) -> Option<&[u8]> {
        if self.pos + self.window_size <= self.data.len() {
            let window = &self.data[self.pos..self.pos + self.window_size];
            self.pos += 1;
            Some(window)
        } else {
            None
        }
    }
}
