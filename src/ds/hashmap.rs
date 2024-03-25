pub mod constant {
    pub const INIT_CAP: usize = 100;
    pub const SHINK_CAP: usize = 1000;
    pub const BUCKET_CAP: usize = 10;
    pub const LOW_FACTOR: f64 = 0.25;
    pub const LOAD_FACTOR: f64 = 0.75;
    pub const GROWTH_FACTOR: usize = 2;
}

use constant::*;
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display};
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::sort::bucket;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pair<K, V> {
    key: K,
    value: V,
}

impl<K: Hash + Debug, V: Debug> Display for Pair<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.key, self.value)
    }
}

impl<K: Hash, V> From<(K, V)> for Pair<K, V> {
    fn from((key, value): (K, V)) -> Self {
        Self { key, value }
    }
}

#[derive(Debug, Clone)]
struct Bucket<K, V> {
    pairs: Vec<Pair<K, V>>,
}

impl<K: Hash, V> Default for Bucket<K, V> {
    fn default() -> Self {
        Self { pairs: vec![] }
    }
}

impl<K: Hash + Debug, V: Debug> Display for Bucket<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.pairs.len() {
            0 => write!(f, ""),
            1 => write!(f, "{}", self.pairs[0]),
            _ => {
                write!(f, "{{")?;
                for (i, pair) in self.pairs.iter().enumerate() {
                    write!(
                        f,
                        "{}{}",
                        pair,
                        if i == self.pairs.len() - 1 { "" } else { ", " }
                    )?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl<K: Eq + Hash + Clone, V: Clone> From<&[(K, V)]> for Bucket<K, V> {
    fn from(pairs: &[(K, V)]) -> Self {
        let mut bucket = Self::default();
        for pair in pairs {
            bucket.push(Pair::from(pair.clone()));
        }
        bucket
    }
}

impl<K: Hash, V> Bucket<K, V> {
    fn len(&self) -> usize {
        self.pairs.len()
    }

    fn clear(&mut self) {
        self.pairs.clear()
    }

    fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    fn push(&mut self, pair: Pair<K, V>)
    where K: Eq
     {
        match self.pairs.iter_mut().find(|p| p.key == pair.key) {
            Some(p) => p.value = pair.value,
            None => self.pairs.push(pair),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HashMap<K, V> {
    buckets: Vec<Bucket<K, V>>,
}

impl<K: Hash, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self {
            buckets: Vec::with_capacity(INIT_CAP),
        }
    }
}

impl<K: Hash + Debug, V: Debug> Display for HashMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let last_idx = self
            .buckets
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, v)| !v.pairs.is_empty())
            .map(|(i, _)| i)
            .next();
        match last_idx {
            Some(last) => {
                write!(f, "{{")?;
                for (i, bucket) in self.buckets.iter().enumerate() {
                    if !bucket.is_empty() {
                        write!(f, "{}{}", bucket, if i == last { "" } else { ", " })?;
                    }
                }
                write!(f, "}}")
            }
            None => write!(f, "{{}}"),
        }
    }
}

impl<K: Eq + Hash + Clone, V: Clone> From<&[(K, V)]> for HashMap<K, V> {
    fn from(pairs: &[(K, V)]) -> Self {
        let cap = if pairs.len() < (INIT_CAP as f64 * LOAD_FACTOR) as usize {
            INIT_CAP
        } else {
            (std::cmp::max(pairs.len(), INIT_CAP) + INIT_CAP - 1) / INIT_CAP * INIT_CAP
        };
        let mut map = Self::with_cap(cap);

        for pair in pairs {
            let idx = Self::idx(cap, &pair.0);
            map.buckets[idx].push(Pair::from(pair.clone()));
        }

        map
    }
}

impl<K: Eq + Hash + Clone, V: Clone> From<&HashMap<K, V>> for Vec<(K, V)> {
    fn from(map: &HashMap<K, V>) -> Self {
        let mut v = Vec::with_capacity(map.count());
        for bucket in map.buckets.iter() {
            for pair in bucket.pairs.iter() {
                v.push((pair.key.clone(), pair.value.clone()))
            }
        }

        v
    }
}

impl<K: PartialOrd + Eq + Hash + Clone, V: PartialEq + Clone> PartialEq for HashMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() || self.count() != other.count() {
            return false;
        }

        let mut v1 = self.to_vec();
        let mut v2 = other.to_vec();
        v1.sort_by(|k1, k2| k1.0.partial_cmp(&k2.0).map_or(Ordering::Equal, |r| r));
        v2.sort_by(|k1, k2| k1.0.partial_cmp(&k2.0).map_or(Ordering::Equal, |r| r));

        v1 == v2
    }
}

impl<K: Eq + Hash + Clone, V: Clone> HashMap<K, V> {
    fn migrate(&mut self, new_cap: usize) {
        let mut pairs = Vec::with_capacity(self.count());
        for bucket in self.buckets.iter() {
            for pair in bucket.pairs.iter() {
                pairs.push(pair.clone());
            }
        }
        self.clear();
        self.buckets.reserve(new_cap);
        for pair in pairs {
            let idx = Self::idx(new_cap, &pair.key);
            self.buckets[idx].push(Pair::from(pair));
        }
    }

    fn shrink(&mut self) {
        if self.cap() > SHINK_CAP && self.len() < (self.cap() as f64 * LOW_FACTOR) as usize {
            let base_cap = std::cmp::max(INIT_CAP, self.len() * GROWTH_FACTOR);
            let new_cap = (base_cap + INIT_CAP - 1) / INIT_CAP * INIT_CAP;
            self.migrate(new_cap);
        }
    }

    fn grow(&mut self) {
        if self.len() > (self.cap() as f64 * LOAD_FACTOR) as usize {
            let new_cap = self.cap() * GROWTH_FACTOR;
            self.migrate(new_cap);
        }
    }

    fn idx(cap: usize, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % cap
    }
}

impl<K: Eq + Hash + Clone, V: Clone> HashMap<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_cap(cap: usize) -> Self {
        let mut buckets = Vec::<Bucket<K, V>>::with_capacity(cap);
        for _ in 0..cap {
            buckets.push(Bucket::default())
        }
        Self { buckets }
    }

    pub fn to_vec(&self) -> Vec<(K, V)>
    where
        K: Clone,
        V: Clone,
    {
        Vec::from(self)
    }

    pub fn len(&self) -> usize {
        self.buckets
            .iter()
            .filter(|bucket| bucket.len() != 0)
            .count()
    }

    pub fn cap(&self) -> usize {
        self.buckets.capacity()
    }

    pub fn count(&self) -> usize {
        self.buckets
            .iter()
            .fold(0, |acc, bucket| acc + bucket.pairs.len())
    }

    pub fn clear(&mut self) {
        for bucket in self.buckets.iter_mut() {
            bucket.clear()
        }
        self.buckets.clear();
    }

    pub fn is_empty(&mut self) -> bool {
        self.buckets.iter().all(|bucket| bucket.is_empty())
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let idx = Self::idx(self.cap(), &key);
        let bucket = &self.buckets[idx];
        if bucket.is_empty() {
            None
        } else {
            bucket
                .pairs
                .iter()
                .find_map(|p| if p.key == key { Some(&p.value) } else { None })
        }
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let idx = Self::idx(self.cap(), &key);
        let bucket = &mut self.buckets[idx];
        if bucket.is_empty() {
            None
        } else {
            bucket.pairs.iter_mut().find_map(|p| {
                if p.key == key {
                    Some(&mut p.value)
                } else {
                    None
                }
            })
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let idx = Self::idx(self.cap(), &key);
        self.buckets[idx].push(Pair::from((key, value)));
        if self.buckets[idx].len() > BUCKET_CAP {
            self.grow();
        }
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let idx = Self::idx(self.cap(), &key);
        let bucket = &mut self.buckets[idx];
        if bucket.is_empty() {
            None
        } else {
            let mut idx_p = None;
            for (i, p) in bucket.pairs.iter().enumerate() {
                if &p.key == &key {
                    idx_p = Some(i);
                    break;
                }
            }
            if idx_p.is_none() {
                None
            } else {
                let r = Some(bucket.pairs.remove(idx_p.unwrap()).value);
                self.shrink();
                r
            }
        }
    }
}
