use std::hash::Hash;

const INIT_CAP: usize = 100;
const LOAD_FACTOR: f64 = 0.75;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pair<K, V> {
    key: K,
    value: V,
}

impl<K: Hash + Default, V: Default> Default for Pair<K, V> {
    fn default() -> Self {
        Self {
            key: Default::default(),
            value: Default::default(),
        }
    }
}

impl<K: Hash, V> From<(K, V)> for Pair<K, V> {
    fn from((key, value): (K, V)) -> Self {
        Self { key, value }
    }
}

#[derive(Debug, Clone)]
struct Buckets<K, V> {
    pairs: Vec<Pair<K, V>>,
}

impl<K: Hash, V> Default for Buckets<K, V> {
    fn default() -> Self {
        Self {
            pairs: Default::default(),
        }
    }
}

impl<K: Hash + Clone, V: Clone> From<&[(K, V)]> for Buckets<K, V> {
    fn from(pairs: &[(K, V)]) -> Self {
        let mut buckets = Self::default();
        for pair in pairs {
            buckets.pairs.push(Pair::from(pair.clone()))
        }
        buckets
    }
}

impl<K: Hash + Default, V: Default> Buckets<K, V> {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct HashMap<K, V> {
    buckets: Vec<Buckets<K, V>>,
}

impl<K: Hash, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self {
            buckets: Vec::with_capacity(INIT_CAP),
        }
    }
}

impl<K: Hash, V> From<&[(K, V)]> for HashMap<K, V> {
    fn from(pairs: &[(K, V)]) -> Self {
        todo!()
    }
}

impl<K: Hash, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.buckets.len()
    }

    pub fn cap(&self) -> usize {
        self.buckets.capacity()
    }
}
