use algorithms_rs::ds::hashmap::constant::*;
use algorithms_rs::ds::HashMap;

const TEST_DATA: [(&str, i32); 6] = [("a", 0), ("b", 1), ("c", 2), ("d", 3), ("e", 4), ("f", 5)];

// test trait impls

#[test]
fn test_default() {
    let map = HashMap::<&str, i32>::default();
    assert_eq!(map.len(), 0);
    assert_eq!(map.cap(), INIT_CAP);
}

#[test]
fn test_from() {
    let map = HashMap::from(&TEST_DATA[..]);
    let data = [("c", 2), ("a", 0), ("e", 4), ("b", 1), ("d", 3), ("f", 5)];
    let map2 = HashMap::from(&data[..]);
    assert_eq!(map.len(), TEST_DATA.len());
    assert_eq!(map, map2);
}

// test impls

#[test]
fn test_new() {
    let map = HashMap::<&str, i32>::new();
    assert_eq!(map.len(), 0);
    assert_eq!(map.cap(), INIT_CAP);
}

#[test]
fn test_with_cap() {
    let map = HashMap::<&str, i32>::with_cap(10);
    assert_eq!(map.len(), 0);
    assert_eq!(map.cap(), 10);
}

#[test]
fn test_to_vec() {
    let map = HashMap::from(&TEST_DATA[..]);
    assert_eq!(
        map.to_vec().sort_by(|k1, k2| k1.0.cmp(&k2.0)),
        TEST_DATA.to_vec().sort_by(|k1, k2| k1.0.cmp(&k2.0))
    );
}

#[test]
fn test_len_cap_count() {
    let map = HashMap::from(&TEST_DATA[..]);
    assert_eq!(map.len(), 6);
    assert_eq!(map.cap(), INIT_CAP);
    assert_eq!(map.count(), map.len());
}

#[test]
fn test_clear() {
    let mut map = HashMap::from(&TEST_DATA[..]);
    map.clear();
    assert_eq!(map.len(), 0);
    assert_eq!(map.to_vec(), vec![]);
}

#[test]
fn test_is_empty() {
    let mut map = HashMap::from(&TEST_DATA[..]);
    assert!(!map.is_empty());
    map.clear();
    assert!(map.is_empty());
}

#[test]
fn test_get() {
    let map = HashMap::from(&TEST_DATA[..]);
    assert_eq!(map.get("a"), Some(&0));
    assert_eq!(map.get("qqq"), None);
}

#[test]
fn test_get_mut() {
    let mut map = HashMap::from(&TEST_DATA[..]);
    let v = map.get_mut("a");
    assert_eq!(v, Some(&mut 0));
    *v.unwrap() = 10;
    assert_eq!(map.get("a"), Some(&10));
    assert_eq!(map.get_mut("qqq"), None);
}

#[test]
fn test_insert() {
    let mut map = HashMap::from(&TEST_DATA[..]);
    map.insert("a", 10);
    assert_eq!(map.get("a"), Some(&10));
    map.insert("z", 99);
    assert_eq!(map.len(), TEST_DATA.len() + 1);
    assert_eq!(map.get("z"), Some(&99));
}

#[test]
fn test_remove() {
    let mut map = HashMap::from(&TEST_DATA[..]);
    assert_eq!(map.remove("qqq"), None);
    assert_eq!(map.len(), TEST_DATA.len());
    assert_eq!(map.remove("a"), Some(0));
    assert_eq!(map.len(), TEST_DATA.len() - 1);
}
