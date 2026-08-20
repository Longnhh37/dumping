pub struct Buffer {
    pub data: Vec<u8>,
    pub read_pos: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            read_pos: 0,
        }
    }

    pub fn append(&mut self, slice: &[u8]) {
        self.data.extend_from_slice(slice)
    }

    pub fn consume(&mut self, n: usize) {
        self.read_pos += n;
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data[self.read_pos..]
    }
}
