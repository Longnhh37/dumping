use crate::btree::node::BNode;
use crate::btree::tree::PageStore;
use crate::storage::freelist::FreeList;
use crate::storage::page::PAGE_SIZE;
use crate::storage::pager::Pager;

pub struct DiskStore {
    pager: Pager,
    freelist: FreeList,
}

impl DiskStore {
    pub fn open(path: &str) -> anyhow::Result<Self> {
        let mut pager = Pager::open(path)?;

        if pager.num_pages() == 0 {
            pager.allocate_page()?;
        }

        Ok(Self {
            pager,
            freelist: FreeList::new(),
        })
    }

    pub fn flush(&mut self) -> anyhow::Result<()> {
        self.pager.flush_all()
    }
}

impl PageStore for DiskStore {
    fn get(&mut self, ptr: u64) -> BNode {
        let page = self.pager.fetch_page(ptr).expect("fetch_page failed");
        BNode::from_bytes(page.data.to_vec())
    }

    fn new_page(&mut self, node: BNode) -> u64 {
        let ptr = match self.freelist.pop() {
            Some(id) => id,
            None => self.pager.allocate_page().expect("allocate_page failed"),
        };

        let page = self
            .pager
            .fetch_page_mut(ptr)
            .expect("fetch_page_mut failed");
        let bytes = node.as_bytes();
        debug_assert_eq!(bytes.len(), PAGE_SIZE);
        page.write_bytes(0, bytes);

        ptr
    }

    fn del(&mut self, ptr: u64) {
        self.freelist.push(ptr);
    }
}
