use algorithms_rs::ds::Vector;

const TEST_DATA: [i32; 6] = [0, 1, 2, 3, 4, 5];

// test trait impls

#[test]
fn test_default() {
    let v = Vector::<i32>::default();
    assert_eq!(v.len(), 0);
    assert_eq!(v.cap(), 0);
}

#[test]
fn test_display() {
    let mut v = Vector::from(&TEST_DATA[..]);
    assert_eq!(format!("{}", v), "[0, 1, 2, 3, 4, 5]");
    v.clear();
    assert_eq!(format!("{}", v), "[]");
}

#[test]
fn test_from() {
    let v = Vector::from(TEST_DATA);
    assert_eq!(v.len(), TEST_DATA.len());
    assert_eq!(&v[..], TEST_DATA);
}

#[test]
fn test_eq() {
    let v1 = Vector::from(&TEST_DATA[..]);
    let mut v2 = Vector::from(&TEST_DATA[..]);
    assert_eq!(v1, v2);
    v2.push(10);
    assert_ne!(v1, v2);
}

#[test]
fn test_iter() {
    let v = Vector::from(TEST_DATA);
    for (i, e) in v.into_iter().enumerate() {
        assert_eq!(TEST_DATA[i], e);
    }
}

// test impls

#[test]
fn test_new() {
    let v = Vector::<i32>::new();
    assert_eq!(v.len(), 0);
}

#[test]
fn test_with_cap() {
    let v = Vector::<i32>::with_cap(10);
    assert_eq!(v.len(), 0);
    assert_eq!(v.cap(), 10);
}

#[test]
fn test_clear() {
    let mut v = Vector::from(&TEST_DATA[..]);
    v.clear();
    assert_eq!(v.len(), 0);
    assert_eq!(v[..], []);
}

#[test]
fn test_is_empty() {
    let mut v = Vector::from(&TEST_DATA[..]);
    assert!(!v.is_empty());
    v.clear();
    assert!(v.is_empty());
}

#[test]
fn test_push() {
    let mut v = Vector::new();
    v.push(0);
    v.push(1);
    assert_eq!(v[..], [0, 1]);
}

#[test]
fn test_pop() {
    let mut v = Vector::from(&TEST_DATA[..]);
    assert_eq!(v.pop(), Some(TEST_DATA[TEST_DATA.len() - 1]));
    assert_eq!(v[..], TEST_DATA[..TEST_DATA.len() - 1]);
    v.clear();
    assert_eq!(v.pop(), None);
}

#[test]
fn test_insert() {
    let mut v = Vector::from(&TEST_DATA[..]);
    v.insert(3, 10);
    assert_eq!(v[..], [0, 1, 2, 10, 3, 4, 5]);
}

#[test]
fn test_remove() {
    let mut v = Vector::from(&TEST_DATA[..]);
    assert_eq!(v.remove(3), 3);
}
