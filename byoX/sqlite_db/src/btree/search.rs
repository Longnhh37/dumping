use crate::btree::insert::node_lookup_le;
use crate::btree::node::{BNode, BNODE_LEAF, BNODE_NODE};
use crate::btree::tree::{BTree, PageStore};

pub(crate) fn tree_search<S: PageStore>(
    tree: &mut BTree<S>,
    node: &BNode,
    key: &[u8],
) -> Option<Vec<u8>> {
    let idx = node_lookup_le(node, key);

    match node.btype() {
        BNODE_LEAF => {
            if key == node.get_key(idx) {
                Some(node.get_val(idx).to_vec())
            } else {
                None
            }
        }
        BNODE_NODE => {
            let kptr = node.get_ptr(idx);
            let kid = tree.store.get(kptr);
            tree_search(tree, &kid, key)
        }
        _ => panic!("bad node!"),
    }
}
