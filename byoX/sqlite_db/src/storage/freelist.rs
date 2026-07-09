pub struct FreeList {
    free: Vec<u64>,
}

impl FreeList {
    pub fn new() -> Self {
        Self { free: Vec::new() }
    }

    pub fn push(&mut self, ptr: u64) {
        self.free.push(ptr);
    }

    pub fn pop(&mut self) -> Option<u64> {
        self.free.pop()
    }

    pub fn len(&self) -> usize {
        self.free.len()
    }

    pub fn is_empty(&self) -> bool {
        self.free.is_empty()
    }
}

impl Default for FreeList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop_lifo() {
        let mut fl = FreeList::new();
        fl.push(1);
        fl.push(2);
        fl.push(3);
        assert_eq!(fl.pop(), Some(3));
        assert_eq!(fl.pop(), Some(2));
        assert_eq!(fl.pop(), Some(1));
        assert_eq!(fl.pop(), None);
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut fl = FreeList::new();
        assert!(fl.is_empty());
        fl.push(10);
        assert_eq!(fl.len(), 1);
        assert!(!fl.is_empty());
    }
}
