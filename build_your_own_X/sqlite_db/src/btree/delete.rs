use crate::btree::insert::{node_append_kv, node_append_range, node_lookup_le, node_replace_kid_n};
use crate::btree::node::{BNode, BNODE_LEAF, BNODE_NODE, HEADER};
use crate::btree::tree::{BTree, PageStore};
use crate::storage::page::PAGE_SIZE;

// ---------- Step 1: Delete From Leaf Nodes ----------
pub(crate) fn leaf_delete(new: &mut BNode, old: &BNode, idx: u16) {
    new.set_header(BNODE_LEAF, old.nkeys() - 1);
    node_append_range(new, old, 0, 0, idx);
    node_append_range(new, old, idx, idx + 1, old.nkeys() - (idx + 1));
}

// ---------- Step 2: Recursive Deletion ----------
pub(crate) fn tree_delete<S: PageStore>(
    tree: &mut BTree<S>,
    node: &BNode,
    key: &[u8],
) -> Option<BNode> {
    let idx = node_lookup_le(node, key);

    match node.btype() {
        BNODE_LEAF => {
            if key != node.get_key(idx) {
                return None; // not found
            }
            let mut new = BNode::with_capacity(PAGE_SIZE);
            leaf_delete(&mut new, node, idx);
            Some(new)
        }
        BNODE_NODE => node_delete(tree, node, idx, key),
        _ => panic!("bad node!"),
    }
}

// ---------- Step 3: Handle Internal Nodes ----------
fn node_delete<S: PageStore>(
    tree: &mut BTree<S>,
    node: &BNode,
    idx: u16,
    key: &[u8],
) -> Option<BNode> {
    let kptr = node.get_ptr(idx);
    let kid = tree.store.get(kptr);
    let updated = tree_delete(tree, &kid, key)?;

    tree.store.del(kptr);

    let mut new = BNode::with_capacity(PAGE_SIZE);

    match should_merge(tree, node, idx, &updated) {
        MergeDir::Left(sibling) => {
            let mut merged = BNode::with_capacity(PAGE_SIZE);
            node_merge(&mut merged, &sibling, &updated);
            tree.store.del(node.get_ptr(idx - 1));
            let merged_ptr = tree.store.new_page(merged.clone());
            node_replace_2kid(&mut new, node, idx - 1, merged_ptr, merged.get_key(0));
        }
        MergeDir::Right(sibling) => {
            let mut merged = BNode::with_capacity(PAGE_SIZE);
            node_merge(&mut merged, &updated, &sibling);
            tree.store.del(node.get_ptr(idx + 1));
            let merged_ptr = tree.store.new_page(merged.clone());
            node_replace_2kid(&mut new, node, idx, merged_ptr, merged.get_key(0));
        }
        MergeDir::None => {
            assert!(updated.nkeys() > 0);
            node_replace_kid_n(tree, &mut new, node, idx, &[updated]);
        }
    }

    Some(new)
}

fn node_merge(new: &mut BNode, left: &BNode, right: &BNode) {
    new.set_header(left.btype(), left.nkeys() + right.nkeys());
    node_append_range(new, left, 0, 0, left.nkeys());
    node_append_range(new, right, left.nkeys(), 0, right.nkeys());
}

fn node_replace_2kid(new: &mut BNode, old: &BNode, idx: u16, merged_ptr: u64, merged_key: &[u8]) {
    new.set_header(BNODE_NODE, old.nkeys() - 1);
    node_append_range(new, old, 0, 0, idx);
    node_append_kv(new, idx, merged_ptr, merged_key, &[]);
    node_append_range(new, old, idx + 1, idx + 2, old.nkeys() - (idx + 2));
}

// ---------- Step 4: The Conditions for Merging ----------

enum MergeDir {
    Left(BNode),
    Right(BNode),
    None,
}

fn should_merge<S: PageStore>(
    tree: &mut BTree<S>,
    node: &BNode,
    idx: u16,
    updated: &BNode,
) -> MergeDir {
    if updated.nbytes() as usize > PAGE_SIZE / 4 {
        return MergeDir::None;
    }

    if idx > 0 {
        let sibling = tree.store.get(node.get_ptr(idx - 1));
        let merged = sibling.nbytes() as usize + updated.nbytes() as usize - HEADER as usize;
        if merged <= PAGE_SIZE {
            return MergeDir::Left(sibling);
        }
    }

    if idx + 1 < node.nkeys() {
        let sibling = tree.store.get(node.get_ptr(idx + 1));
        let merged = sibling.nbytes() as usize + updated.nbytes() as usize - HEADER as usize;
        if merged <= PAGE_SIZE {
            return MergeDir::Right(sibling);
        }
    }

    MergeDir::None
}
