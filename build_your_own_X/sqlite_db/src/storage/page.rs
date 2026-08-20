pub const PAGE_SIZE: usize = 4096;

pub struct Page {
    pub id: u64,
    pub data: [u8; PAGE_SIZE],
    pub dirty: bool,
}

impl Page {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            data: [0; PAGE_SIZE],
            dirty: false,
        }
    }

    pub fn offset(&self) -> u64 {
        self.id * PAGE_SIZE as u64
    }

    pub fn read_bytes(&self, offset: usize, len: usize) -> &[u8] {
        &self.data[offset..offset + len]
    }

    pub fn write_bytes(&mut self, offset: usize, src: &[u8]) {
        self.data[offset..offset + src.len()].copy_from_slice(src);
    }
}
