use crate::btree::node::{BNode, BNODE_LEAF};
use crate::btree::tree::{BTree, PageStore};
pub use crate::btree::split::node_split3;

// ---------- Step 1: Look Up the Key ----------
pub fn node_lookup_le(node: &BNode, key: &[u8]) -> u16 {
    let nkeys = node.nkeys();
    let mut found: u16 = 0;

    for i in 1..nkeys {
        let cmp = node.get_key(i).cmp(key);
        if cmp != std::cmp::Ordering::Greater {
            found = i;
        }
        if cmp != std::cmp::Ordering::Less {
            break;
        }
    }
    found
}

// ---------- Step 2: Update Leaf Nodes ----------
pub fn leaf_insert(new: &mut BNode, old: &BNode, idx: u16, key: &[u8], val: &[u8]) {
    new.set_header(BNODE_LEAF, old.nkeys() + 1);
    node_append_range(new, old, 0, 0, idx);
    node_append_kv(new, idx, 0, key, val);
    node_append_range(new, old, idx + 1, idx, old.nkeys() - idx);
}

pub fn leaf_update(new: &mut BNode, old: &BNode, idx: u16, key: &[u8], val: &[u8]) {
    new.set_header(BNODE_LEAF, old.nkeys());
    node_append_range(new, old, 0, 0, idx);
    node_append_kv(new, idx, 0, key, val);
    node_append_range(new, old, idx + 1, idx + 1, old.nkeys() - idx - 1);
}

pub(crate) fn node_append_range(new: &mut BNode, old: &BNode, dst_new: u16, src_old: u16, n: u16) {
    assert!(src_old + n <= old.nkeys());
    assert!(dst_new + n <= new.nkeys());

    if n == 0 {
        return;
    }

    // ----- copy pointers -----
    for i in 0..n {
        new.set_ptr(dst_new + i, old.get_ptr(src_old + i));
    }

    // ----- copy offsets -----
    let dst_begin = new.get_offset(dst_new);
    let src_begin = old.get_offset(src_old);
    for i in 1..=n {
        let offset = dst_begin + old.get_offset(src_old + i) - src_begin;
        new.set_offset(dst_new + i, offset);
    }

    // ----- copy KV bytes (raw memcpy vùng liên tục) -----
    let begin = old.kv_pos(src_old);
    let end = old.kv_pos(src_old + n);
    let dst_pos = new.kv_pos(dst_new);
    new.copy_kv_bytes(dst_pos, old.kv_bytes(begin, end));
}

pub(crate) fn node_append_kv(new: &mut BNode, idx: u16, ptr: u64, key: &[u8], val: &[u8]) {
    new.set_ptr(idx, ptr);

    let pos = new.kv_pos(idx);
    new.write_kv(pos, key, val);

    new.set_offset(
        idx + 1,
        new.get_offset(idx) + 4 + key.len() as u16 + val.len() as u16,
    );
}

// ---------- Step 3: Recursive Insertion ----------
pub fn tree_insert<S: PageStore>(
    tree: &mut BTree<S>,
    node: &BNode,
    key: &[u8],
    val: &[u8],
) -> BNode {
    let mut new = BNode::with_capacity(2 * crate::storage::page::PAGE_SIZE);

    let idx = node_lookup_le(node, key);

    match node.btype() {
        crate::btree::node::BNODE_LEAF => {
            if key == node.get_key(idx) {
                leaf_update(&mut new, node, idx, key, val);
            } else {
                leaf_insert(&mut new, node, idx + 1, key, val);
            }
        }
        crate::btree::node::BNODE_NODE => {
            node_insert(tree, &mut new, node, idx, key, val);
        }
        _ => panic!("bad node!"),
    }

    new
}

// ---------- Step 4: Handle Internal Nodes ----------
fn node_insert<S: PageStore>(
    tree: &mut BTree<S>,
    new: &mut BNode,
    node: &BNode,
    idx: u16,
    key: &[u8],
    val: &[u8],
) {
    let kptr = node.get_ptr(idx);
    let knode = tree.store.get(kptr);
    tree.store.del(kptr);

    let knode = tree_insert(tree, &knode, key, val);
    let split_result = node_split3(knode);

    node_replace_kid_n(tree, new, node, idx, &split_result);
}

// ---------- Step 6: Update Internal Nodes ----------
pub(crate) fn node_replace_kid_n<S: PageStore>(
    tree: &mut BTree<S>,
    new: &mut BNode,
    old: &BNode,
    idx: u16,
    kids: &[BNode],
) {
    let inc = kids.len() as u16;
    new.set_header(crate::btree::node::BNODE_NODE, old.nkeys() + inc - 1);

    node_append_range(new, old, 0, 0, idx);

    for (i, kid) in kids.iter().enumerate() {
        let kid_ptr = tree.store.new_page(kid.clone());
        node_append_kv(new, idx + i as u16, kid_ptr, kid.get_key(0), &[]);
    }

    node_append_range(new, old, idx + inc, idx + 1, old.nkeys() - (idx + 1));
}
