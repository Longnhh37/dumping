use crate::btree::node::BNode;
use crate::storage::page::PAGE_SIZE;

// ---------- Step 5: Split Big Nodes ----------
fn node_split2(left: &mut BNode, right: &mut BNode, old: &BNode) {
    assert!(old.nkeys() >= 2);

    let mut n_left = old.nkeys() / 2;
    let try_right_bytes = |n_left: u16| -> u16 {
        let n_right = old.nkeys() - n_left;
        let header_and_meta = 4u16 + 8 * n_right + 2 * n_right;
        let kv_bytes = old.kv_pos(old.nkeys()) as u16 - old.kv_pos(n_left) as u16;
        header_and_meta + kv_bytes
    };

    while n_left > 1 && try_right_bytes(n_left) as usize > PAGE_SIZE {
        n_left += 1;
    }
    n_left = n_left.min(old.nkeys() - 1);

    let n_right = old.nkeys() - n_left;

    left.set_header(old.btype(), n_left);
    node_append_range_pub(left, old, 0, 0, n_left);

    right.set_header(old.btype(), n_right);
    node_append_range_pub(right, old, 0, n_left, n_right);

    debug_assert!(right.nbytes() as usize <= PAGE_SIZE);
}

pub fn node_split3(mut old: BNode) -> Vec<BNode> {
    if old.nbytes() as usize <= PAGE_SIZE {
        old.truncate_to_page();
        return vec![old];
    }

    let mut left = BNode::with_capacity(2 * PAGE_SIZE);
    let mut right = BNode::with_capacity(PAGE_SIZE);
    node_split2(&mut left, &mut right, &old);

    if left.nbytes() as usize <= PAGE_SIZE {
        left.truncate_to_page();
        return vec![left, right];
    }

    let mut leftleft = BNode::with_capacity(PAGE_SIZE);
    let mut middle = BNode::with_capacity(PAGE_SIZE);
    node_split2(&mut leftleft, &mut middle, &left);

    debug_assert!(leftleft.nbytes() as usize <= PAGE_SIZE);
    vec![leftleft, middle, right]
}

use crate::btree::insert::node_append_range as node_append_range_pub;
