use algorithms_rs::ds::SqList;

const LEN: usize = 6;
const TEST_DATA: [i32; LEN] = [0, 1, 2, 3, 4, 5];

// test trait impls

#[test]
fn test_default() {
    let list = SqList::<i32>::default();
    assert_eq!(list.len(), 0);
}

#[test]
fn test_from() {
    let list = SqList::from(TEST_DATA);
    assert_eq!(list.len(), LEN);
    assert_eq!(list, TEST_DATA);
}

#[test]
fn test_eq() {
    let list1 = SqList::from(TEST_DATA);
    let list2 = SqList::from(TEST_DATA);
    assert_eq!(list1, list2);
    assert_eq!(list1, TEST_DATA);
    assert_eq!(list1, &TEST_DATA[..]);
    assert_eq!(list1, TEST_DATA.to_vec());
    assert_ne!(list1, &TEST_DATA[1..]);
}

#[test]
fn test_display() {
    let mut list = SqList::from(TEST_DATA);
    assert_eq!(format!("{}", list), "[0, 1, 2, 3, 4, 5]");
    list.clear();
    assert_eq!(format!("{}", list), "[]");
}

// test impls

#[test]
fn test_new() {
    let list = SqList::<i32>::new();
    assert_eq!(list.len(), 0);
}

#[test]
fn test_clear() {
    let mut list = SqList::from(TEST_DATA);
    list.clear();
    assert_eq!(list.len(), 0);
    assert_eq!(list, []);
}

#[test]
fn test_contains() {
    let mut list = SqList::from(TEST_DATA);
    assert!(list.contains(&3));
    list.clear();
    assert!(!list.contains(&5));
}

#[test]
fn test_find() {
    let list = SqList::from(TEST_DATA);
    assert_eq!(list.find(&3), Some(3));
    assert_eq!(list.find(&99), None);
}

#[test]
fn test_find_all() {
    let list = SqList::from(TEST_DATA);
    assert_eq!(list.find_all(&3), vec![3]);
    assert_eq!(list.find_all(&99), vec![]);
}

#[test]
fn test_insert() {
    let mut list = SqList::from(TEST_DATA);
    assert!(list.insert(0, 10));
    assert_eq!(list[0], 10);
    assert_eq!(list.len(), LEN + 1);

    assert!(list.insert(6, 10));
    assert_eq!(list[6], 10);
    assert_eq!(list.len(), LEN + 2);

    assert!(list.insert(3, 10));
    assert_eq!(list[3], 10);
    assert_eq!(list.len(), LEN + 3);

    assert!(!list.insert(99, 10));
    assert_eq!(list.len(), LEN + 3);
}

#[test]
fn test_remove() {
    let mut list = SqList::from(TEST_DATA);
    assert_eq!(list.remove(0), Some(0));
    assert_eq!(list.len(), LEN - 1);
    assert_eq!(list, &TEST_DATA[1..]);

    assert_eq!(list.remove(4), Some(5));
    assert_eq!(list.len(), LEN - 2);
    assert_eq!(list, &TEST_DATA[1..5]);

    assert_eq!(list.remove(99), None);
    assert_eq!(list.len(), LEN - 2);

    list.clear();
    assert_eq!(list.remove(0), None);
}

#[test]
fn test_push_front() {
    let mut list = SqList::from(TEST_DATA);
    assert!(list.push_front(10));
    assert_eq!(list[0], 10);
    assert_eq!(list.len(), LEN + 1);
}

#[test]
fn test_push_back() {
    let mut list = SqList::from(TEST_DATA);
    assert!(list.push_back(10));
    assert_eq!(list[6], 10);
    assert_eq!(list.len(), LEN + 1);
}

#[test]
fn test_pop_front() {
    let mut list = SqList::from(TEST_DATA);
    assert_eq!(list.pop_front(), Some(0));
    assert_eq!(list.len(), LEN - 1);
    assert_eq!(list, &TEST_DATA[1..]);

    list.clear();
    assert_eq!(list.pop_front(), None);
}

#[test]
fn test_pop_back() {
    let mut list = SqList::from(TEST_DATA);
    assert_eq!(list.pop_back(), Some(5));
    assert_eq!(list.len(), LEN - 1);
    assert_eq!(list, &TEST_DATA[..LEN - 1]);

    list.clear();
    assert_eq!(list.pop_back(), None);
}
