use algorithms_rs::ds::Vector;

const LEN: usize = 6;
const TEST_DATA: [i32; LEN] = [0, 1, 2, 3, 4, 5];

// test trait impls

#[test]
fn test_default() {
    let v = Vector::<i32>::default();
    assert_eq!(v.len(), 0);
    assert_eq!(v.cap(), 0);
}

#[test]
fn test_from() {
    let v = Vector::from(TEST_DATA);
    assert_eq!(v.len(), LEN);
    assert_eq!(v, TEST_DATA);
}

#[test]
fn test_eq() {
    let v1 = Vector::from(TEST_DATA);
    let v2 = Vector::from(TEST_DATA);
    assert_eq!(v1, v2);
    assert_eq!(v1, TEST_DATA);
    assert_ne!(v1, &TEST_DATA[1..]);
}

#[test]
fn test_display() {
    let mut v = Vector::from(TEST_DATA);
    assert_eq!(format!("{}", v), "[0, 1, 2, 3, 4, 5]");
    v.clear();
    assert_eq!(format!("{}", v), "[]");
}

#[test]
fn test_into_iter() {
    let v = Vector::from(TEST_DATA);
    let mut iter = v.into_iter();
    for e in TEST_DATA.iter() {
        assert_eq!(iter.next(), Some(*e));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter() {
    let v = Vector::from(TEST_DATA);
    let mut iter = v.iter();
    for e in TEST_DATA.iter() {
        assert_eq!(iter.next(), Some(e));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_mut() {
    let mut v = Vector::from(TEST_DATA);
    for e in v.iter_mut() {
        *e += 1;
    }
    assert_eq!(v, [1, 2, 3, 4, 5, 6]);
}

// test impls

#[test]
fn test_new() {
    let v = Vector::<i32>::new();
    assert_eq!(v.len(), 0);
    assert_eq!(v.cap(), 0);
}

#[test]
fn test_with_cap() {
    let v = Vector::<i32>::with_cap(10);
    assert_eq!(v.len(), 0);
    assert_eq!(v.cap(), 10);
}

#[test]
fn test_clear() {
    let mut v = Vector::from(TEST_DATA);
    v.clear();
    assert_eq!(v.len(), 0);
    assert_eq!(v, []);
}

#[test]
fn test_is_empty() {
    let mut v = Vector::from(TEST_DATA);
    assert!(!v.is_empty());
    v.clear();
    assert!(v.is_empty());
}

#[test]
fn test_swap() {
    let mut v = Vector::from(TEST_DATA);
    v.swap(0, LEN - 1);
    assert_eq!(v[0], TEST_DATA[LEN - 1]);
}

#[test]
fn test_push() {
    let mut v = Vector::new();
    v.push(0);
    v.push(1);
    assert_eq!(v, &TEST_DATA[..2]);
}

#[test]
fn test_pop() {
    let mut v = Vector::from(TEST_DATA);
    assert_eq!(v.pop(), Some(TEST_DATA[LEN - 1]));
    assert_eq!(v, &TEST_DATA[..LEN - 1]);
    v.clear();
    assert_eq!(v.pop(), None);
}

#[test]
fn test_insert() {
    let mut v = Vector::from(TEST_DATA);
    v.insert(3, 10);
    assert_eq!(v, [0, 1, 2, 10, 3, 4, 5]);
}

#[test]
fn test_remove() {
    let mut v = Vector::from(TEST_DATA);
    assert_eq!(v.remove(3), 3);
}
