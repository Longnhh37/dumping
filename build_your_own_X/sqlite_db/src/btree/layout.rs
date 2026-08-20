use crate::storage::page::PAGE_SIZE;

pub const BNODE_NODE: u16 = 1;
pub const BNODE_LEAF: u16 = 2;

pub const HEADER: u16 = 4;
pub const PTR_SIZE: u16 = 8;
pub const OFFSET_SIZE: u16 = 2;

pub const BTREE_MAX_KEY_SIZE: u16 = 1000;
pub const BTREE_MAX_VAL_SIZE: u16 = 3000;

const _: () = assert!(
    (HEADER + PTR_SIZE + OFFSET_SIZE + 4 + BTREE_MAX_KEY_SIZE + BTREE_MAX_VAL_SIZE) as usize
        <= PAGE_SIZE
);
