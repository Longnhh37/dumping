use super::{file::DbFile, page::Page};
use anyhow::{Result, bail};
use std::collections::HashMap;

const DEFAULT_CACHE_CAPACITY: usize = 64; // maximum num of pages in RAM

pub struct Pager {
    file: DbFile,
    cache: HashMap<u64, Page>, // page_id -> Page
    lru: Vec<u64>,             // front = most recent, back = LRU
    capacity: usize,
    num_pages: u64, // total number of pages on disk
}

impl Pager {
    pub fn open(path: &str) -> Result<Self> {
        let mut file = DbFile::open(path)?;
        let num_pages = file.num_pages()?;

        Ok(Self {
            file,
            cache: HashMap::new(),
            lru: Vec::new(),
            capacity: DEFAULT_CACHE_CAPACITY,
            num_pages,
        })
    }

    // return page reference. Load from disk if not in cache
    pub fn fetch_page(&mut self, page_id: u64) -> Result<&Page> {
        if page_id >= self.num_pages {
            bail!(
                "page_id {} out of range (total: {})",
                page_id,
                self.num_pages
            );
        }

        // cache hit: update LRU order
        if !self.cache.contains_key(&page_id) {
            self.load_page(page_id)?;
        }

        self.touch(page_id); // move page to first position in LRU
        Ok(self.cache.get(&page_id).unwrap())
    }

    pub fn fetch_page_mut(&mut self, page_id: u64) -> Result<&mut Page> {
        if page_id >= self.num_pages {
            bail!(
                "page_id {} out of range (total: {})",
                page_id,
                self.num_pages
            );
        }

        // cache hit: update LRU order
        if !self.cache.contains_key(&page_id) {
            self.load_page(page_id)?;
        }

        self.touch(page_id); // move page to first position in LRU
        let page = self.cache.get_mut(&page_id).unwrap();
        page.dirty = true;

        Ok(page)
    }

    fn load_page(&mut self, page_id: u64) -> Result<()> {
        // evict if cache is full
        if self.cache.len() >= self.capacity {
            self.evict_lru()?;
        }

        let mut page = Page::new(page_id);
        self.file.read_at(page.offset(), &mut page.data)?;
        self.cache.insert(page_id, page);
        self.lru.insert(0, page_id);

        Ok(())
    }

    // ----- LRU helpers ----------------------------------------------------
    // bring page_id to top (most recently used)
    fn touch(&mut self, page_id: u64) {
        self.lru.retain(|&id| id != page_id);
        self.lru.insert(0, page_id);
    }

    // delete least used page (end of vec), flush to disk if dirty
    fn evict_lru(&mut self) -> Result<()> {
        let victim_id = match self.lru.pop() {
            // back = LRU
            Some(id) => id,
            None => bail!("cache empty, nothing to evict"),
        };

        if let Some(page) = self.cache.remove(&victim_id)
            && page.dirty
        {
            self.file.write_at(page.offset(), &page.data)?;
        }

        Ok(())
    }

    pub fn num_pages(&self) -> u64 {
        self.num_pages
    }

    // allocate new page at end of file, return page_id
    pub fn allocate_page(&mut self) -> Result<u64> {
        let new_id = self.num_pages;
        self.num_pages += 1;

        // evict before adding new page into cache
        if self.cache.len() >= self.capacity {
            self.evict_lru()?;
        }

        let page = Page::new(new_id);
        // write zeros to disk so that file size is matched
        self.file.write_at(page.offset(), &page.data)?;

        self.cache.insert(new_id, page);
        self.lru.insert(0, new_id);

        Ok(new_id)
    }

    // flush dirty pages to disk, then sync
    pub fn flush_all(&mut self) -> Result<()> {
        for page in self.cache.values_mut().filter(|p| p.dirty) {
            self.file.write_at(page.offset(), &page.data)?;
            page.dirty = false;
        }
        self.file.sync()?;
        Ok(())
    }
}

impl Drop for Pager {
    fn drop(&mut self) {
        let _ = self.flush_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn temp_pager() -> (Pager, NamedTempFile) {
        let tmp = NamedTempFile::new().unwrap();
        let pager = Pager::open(tmp.path().to_str().unwrap()).unwrap();
        (pager, tmp)
    }

    #[test]
    fn test_allocate_increases_num_pages() {
        let (mut pager, _tmp) = temp_pager();
        assert_eq!(pager.num_pages, 0);
        pager.allocate_page().unwrap();
        assert_eq!(pager.num_pages, 1);
    }

    #[test]
    fn test_write_page_persists_after_flush_and_reopen() {
        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_str().unwrap().to_string();

        {
            let mut pager = Pager::open(&path).unwrap();
            let id = pager.allocate_page().unwrap();
            let page = pager.fetch_page_mut(id).unwrap();
            page.data[0..5].copy_from_slice(b"hello");
            pager.flush_all().unwrap();
        }

        let mut pager2 = Pager::open(&path).unwrap();
        let page = pager2.fetch_page(0).unwrap();
        assert_eq!(&page.data[0..5], b"hello");
    }

    #[test]
    fn test_eviction_flushes_dirty_page() {
        let (mut pager, tmp) = temp_pager();
        pager.capacity = 2; // cache nhỏ để force eviction

        let id0 = pager.allocate_page().unwrap();
        let _id1 = pager.allocate_page().unwrap();

        // Dirty page 0
        pager.fetch_page_mut(id0).unwrap().data[0] = 42;

        // Allocate page thứ 3 → evict page ít dùng nhất
        let _id2 = pager.allocate_page().unwrap();

        // Mở lại, page 0 phải có data[0] == 42
        drop(pager);
        let path = tmp.path().to_str().unwrap().to_string();
        let mut pager2 = Pager::open(&path).unwrap();
        let page = pager2.fetch_page(id0).unwrap();
        assert_eq!(page.data[0], 42);
    }

    #[test]
    fn test_fetch_out_of_range_returns_error() {
        let (mut pager, _tmp) = temp_pager();
        let result = pager.fetch_page(999);
        assert!(result.is_err());
    }
}
