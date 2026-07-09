use crate::btree::delete::tree_delete;
use crate::btree::insert::{node_append_kv, node_split3, tree_insert};
use crate::btree::node::{BNode, BNODE_LEAF, BNODE_NODE, BTREE_MAX_KEY_SIZE, BTREE_MAX_VAL_SIZE};
use crate::btree::search::tree_search;
use crate::storage::page::PAGE_SIZE;

pub trait PageStore {
    fn get(&mut self, ptr: u64) -> BNode;
    fn new_page(&mut self, node: BNode) -> u64;
    fn del(&mut self, ptr: u64);
}

pub struct BTree<S: PageStore> {
    root: u64,
    pub(crate) store: S,
}

impl<S: PageStore> BTree<S> {
    pub fn new(store: S) -> Self {
        Self { root: 0, store }
    }

    pub fn get(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        assert!(!key.is_empty());
        assert!(key.len() <= BTREE_MAX_KEY_SIZE as usize);

        if self.root == 0 {
            return None;
        }

        let node = self.store.get(self.root);
        tree_search(self, &node, key)
    }

    pub fn insert(&mut self, key: &[u8], val: &[u8]) {
        assert!(!key.is_empty());
        assert!(key.len() <= BTREE_MAX_KEY_SIZE as usize);
        assert!(val.len() <= BTREE_MAX_VAL_SIZE as usize);

        if self.root == 0 {
            let mut root = BNode::with_capacity(PAGE_SIZE);
            root.set_header(BNODE_LEAF, 2);
            node_append_kv(&mut root, 0, 0, &[], &[]);
            node_append_kv(&mut root, 1, 0, key, val);
            self.root = self.store.new_page(root);
            return;
        }

        let node = self.store.get(self.root);
        self.store.del(self.root);

        let node = tree_insert(self, &node, key, val);
        let splitted = node_split3(node);

        if splitted.len() > 1 {
            let mut root = BNode::with_capacity(PAGE_SIZE);
            root.set_header(BNODE_NODE, splitted.len() as u16);
            for (i, knode) in splitted.iter().enumerate() {
                let key0 = knode.get_key(0).to_vec();
                let ptr = self.store.new_page(knode.clone());
                node_append_kv(&mut root, i as u16, ptr, &key0, &[]);
            }
            self.root = self.store.new_page(root);
        } else {
            self.root = self.store.new_page(splitted[0].clone());
        }
    }

    pub fn delete(&mut self, key: &[u8]) -> bool {
        assert!(!key.is_empty());
        assert!(key.len() <= BTREE_MAX_KEY_SIZE as usize);

        if self.root == 0 {
            return false;
        }

        let root = self.store.get(self.root);
        let updated = match tree_delete(self, &root, key) {
            Some(n) => n,
            None => return false,
        };

        self.store.del(self.root);

        if updated.btype() == BNODE_NODE && updated.nkeys() == 1 {
            self.root = updated.get_ptr(0);
        } else {
            self.root = self.store.new_page(updated);
        }

        true
    }
}
