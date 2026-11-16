use std::collections::BTreeMap;
use std::hash::{DefaultHasher, Hash, Hasher};

fn hash_of<T: Hash + ?Sized>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

struct HashRing {
    ring: BTreeMap<u64, String>,
    vnodes: usize,
}

impl HashRing {
    fn new(vnodes: usize) -> Self {
        Self {
            ring: BTreeMap::new(),
            vnodes,
        }
    }

    fn add_node(&mut self, node: &str) {
        for i in 0..self.vnodes {
            self.ring
                .insert(hash_of(&format!("{node}#{i}")), node.to_owned());
        }
    }

    fn remove_node(&mut self, node: &str) {
        for i in 0..self.vnodes {
            self.ring.remove(&hash_of(&format!("{node}#{i}")));
        }
    }

    fn get_node(&self, key: &str) -> Option<&str> {
        let h = hash_of(key);
        self.ring
            .range(h..)
            .next()
            .or_else(|| self.ring.iter().next())
            .map(|(_, node)| node.as_str())
    }
}

fn main() {
    let mut ring = HashRing::new(100);
    for n in ["node-a", "node-b", "node-c", "node-d"] {
        ring.add_node(n);
    }

    let keys: Vec<String> = (0..10_000).map(|i| format!("key-{i}")).collect();
    let before: Vec<_> = keys
        .iter()
        .map(|k| ring.get_node(k).unwrap().to_owned())
        .collect();

    ring.add_node("node-e");

    let moved = keys
        .iter()
        .zip(&before)
        .filter(|(k, old)| ring.get_node(k).unwrap() != old.as_str())
        .count();

    println!("moved {moved} / {} keys", keys.len());
}
