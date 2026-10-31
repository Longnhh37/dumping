use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

struct Cache<K: Eq + Hash + Clone, V> {
    map: HashMap<K, V>,
    queue: VecDeque<K>,
    capacity: usize,
}

impl<K: Eq + Hash + Clone, V> Cache<K, V> {
    fn new(cap: usize) -> Self {
        Self {
            map: HashMap::with_capacity(cap),
            queue: VecDeque::with_capacity(cap),
            capacity: cap,
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    fn insert(&mut self, key: K, value: V) {
        if self.len() >= self.capacity {
            self.evict();
        }
        self.queue.push_back(key.clone());
        self.map.insert(key, value);
    }

    fn len(&self) -> usize {
        self.queue.len()
    }

    fn evict(&mut self) {
        if let Some(k) = self.queue.pop_front() {
            self.map.remove(&k);
        }
    }
}
