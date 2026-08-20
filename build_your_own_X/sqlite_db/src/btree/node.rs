use crate::storage::page::PAGE_SIZE;

pub use crate::btree::layout::{
    BNODE_LEAF, BNODE_NODE, BTREE_MAX_KEY_SIZE, BTREE_MAX_VAL_SIZE, HEADER, OFFSET_SIZE, PTR_SIZE,
};

#[derive(Clone)]
pub struct BNode {
    data: Vec<u8>,
}

impl BNode {
    pub fn new() -> Self {
        Self {
            data: vec![0; PAGE_SIZE]
        }
    }

    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self {
            data
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn with_capacity(size: usize) -> Self {
        Self { data: vec![0; size] }
    }

    pub fn kv_bytes(&self, begin: usize, end: usize) -> &[u8] {
        &self.data[begin..end]
    }

    pub fn copy_kv_bytes(&mut self, pos: usize, src: &[u8]) {
        self.data[pos..pos + src.len()].copy_from_slice(src);
    }

    pub fn write_kv(&mut self, pos: usize, key: &[u8], val: &[u8]) {
        self.data[pos..pos + 2].copy_from_slice(&(key.len() as u16).to_le_bytes());
        self.data[pos + 2..pos + 4].copy_from_slice(&(val.len() as u16).to_le_bytes());
        self.data[pos + 4..pos + 4 + key.len()].copy_from_slice(key);
        self.data[pos + 4 + key.len()..pos + 4 + key.len() + val.len()].copy_from_slice(val);
    }

    pub fn truncate_to_page(&mut self) {
        self.data.truncate(crate::storage::page::PAGE_SIZE);
    }

    // ---------- header ----------
    pub fn btype(&self) -> u16 {
        u16::from_le_bytes(self.data[0..2].try_into().unwrap())

    }

    pub fn nkeys(&self) -> u16 {
        u16::from_le_bytes(self.data[2..4].try_into().unwrap())
    }

    pub fn set_header(&mut self, btype: u16, nkeys: u16) {
        self.data[0..2].copy_from_slice(&btype.to_le_bytes());
        self.data[2..4].copy_from_slice(&nkeys.to_le_bytes());
    }

    // ---------- pointers ----------
    pub fn get_ptr(&self, idx: u16) -> u64 {
        assert!(idx < self.nkeys());
        let pos = (HEADER + PTR_SIZE * idx) as usize;
        u64::from_le_bytes(self.data[pos..pos + PTR_SIZE as usize].try_into().unwrap())
    }

    pub fn set_ptr(&mut self, idx: u16, val: u64) {
        assert!(idx < self.nkeys());
        let pos = (HEADER + PTR_SIZE * idx) as usize;
        self.data[pos..pos + PTR_SIZE as usize].copy_from_slice(&val.to_le_bytes())
    }

    // ---------- offset list ----------
    fn offset_pos(&self, idx: u16) -> usize {
        assert!(1 <= idx && idx <= self.nkeys());
        (HEADER + PTR_SIZE * self.nkeys() + OFFSET_SIZE * (idx - 1)) as usize
    }

    pub fn get_offset(&self, idx: u16) -> u16 {
        if idx == 0 {
            return 0;
        }
        let pos = self.offset_pos(idx);
        u16::from_le_bytes(self.data[pos..pos + OFFSET_SIZE as usize].try_into().unwrap())
    }

    pub fn set_offset(&mut self, idx: u16, offset: u16) {
        let pos = self.offset_pos(idx);
        self.data[pos..pos + OFFSET_SIZE as usize].copy_from_slice(&offset.to_le_bytes());
    }

    // ---------- key-values ----------
    pub fn kv_pos(&self, idx: u16) -> usize {
        let nkeys = self.nkeys();
        assert!(idx <= nkeys);
        (HEADER + PTR_SIZE * nkeys + OFFSET_SIZE * nkeys) as usize
        + self.get_offset(idx) as usize
    }

    pub fn get_key(&self, idx: u16) -> &[u8] {
        assert!(idx < self.nkeys());
        let pos = self.kv_pos(idx);
        let klen = u16::from_le_bytes(self.data[pos..pos + 2].try_into().unwrap());
        &self.data[pos + 4..pos + 4 + klen as usize]
    }

    pub fn get_val(&self, idx: u16) -> &[u8] {
        assert!(idx < self.nkeys());
        let pos = self.kv_pos(idx);
        let klen = u16::from_le_bytes(self.data[pos..pos + 2].try_into().unwrap());
        let vlen = u16::from_le_bytes(self.data[pos + 2..pos + 4].try_into().unwrap());
        &self.data[pos + 4 + klen as usize..pos + 4 + klen as usize + vlen as usize]
    }

    // ---------- node size ----------
    pub fn nbytes(&self) -> u16 {
        self.kv_pos(self.nkeys()) as u16
    }
}
