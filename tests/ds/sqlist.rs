use algorithms_rs::ds::SqList;

const TEST_DATA: [i32; 6] = [0, 1, 2, 3, 4, 5];

// test trait impls

#[test]
fn test_default() {
    let list = SqList::<i32>::default();
    assert_eq!(list[0], 0);
    assert_eq!(list.len(), 0);
}

#[test]
fn test_display() {
    let mut list = SqList::from(&TEST_DATA[..]);
    assert_eq!(format!("{}", list), "[0, 1, 2, 3, 4, 5]");
    list.clear();
    assert_eq!(format!("{}", list), "[]");
}

#[test]
fn test_from() {
    let list = SqList::from(&TEST_DATA[..]);
    assert_eq!(list.len(), TEST_DATA.len());
    assert_eq!(&list[0..list.len()], TEST_DATA);
}

#[test]
fn test_eq() {
    let list1 = SqList::from(&TEST_DATA[..]);
    let list2 = SqList::from(&TEST_DATA[..]);
    assert_eq!(list1, list2);
}

#[test]
fn test_as_ref() {
    let list = SqList::from(&TEST_DATA[..]);
    assert_eq!(list.as_ref(), &TEST_DATA[..]);
}

#[test]
fn test_as_mut() {
    let mut list = SqList::from(&TEST_DATA[..]);
    list.as_mut()[0] = 10;
    assert_eq!(list[0], 10);
}

#[test]
fn test_index() {
    let list = SqList::from(&TEST_DATA[..]);
    assert_eq!(list[0], 0);
    assert_eq!(list[0..3], [0, 1, 2]);
}

#[test]
fn test_index_mut() {
    let mut list = SqList::from(&TEST_DATA[..]);
    list[0] = 10;
    assert_eq!(list[0], 10);
    for e in &mut list[1..4] {
        *e += 10;
    }
    assert_eq!(list[1..4], [11, 12, 13]);
}

// test impls

#[test]
fn test_new() {
    let list = SqList::<i32>::new();
    assert_eq!(list.len(), 0);
}

#[test]
fn test_swap() {
    let mut list = SqList::from(&TEST_DATA[..]);
    list.swap(0, 5);
    assert_eq!(list[0..list.len()], [5, 1, 2, 3, 4, 0]);

    list.swap(0, 99);
    assert_eq!(list[0..list.len()], [5, 1, 2, 3, 4, 0]);
}

#[test]
fn test_reverse() {
    let mut list = SqList::from(&TEST_DATA[..]);
    list.reverse();
    assert_eq!(list[0..list.len()], [5, 4, 3, 2, 1, 0]);
}

#[test]
fn test_clear() {
    let mut list = SqList::from(&TEST_DATA[..]);
    list.clear();
    assert_eq!(list.len(), 0);
    assert_eq!(list[0..list.len()], []);
}

#[test]
fn test_is_empty() {
    let mut list = SqList::from(&TEST_DATA[..]);
    assert!(!list.is_empty());
    list.clear();
    assert!(list.is_empty());
}

#[test]
fn test_get() {
    let list = SqList::from(&TEST_DATA[..]);
    assert_eq!(list.get(0), Some(0));
    assert_eq!(list.get(6), None);
}

#[test]
fn test_first() {
    let mut list = SqList::from(&TEST_DATA[..]);
    assert_eq!(list.first(), Some(0));
    list.clear();
    assert_eq!(list.first(), None);
}

#[test]
fn test_last() {
    let mut list = SqList::from(&TEST_DATA[..]);
    assert_eq!(list.last(), Some(5));
    list.clear();
    assert_eq!(list.last(), None);
}

#[test]
fn test_set() {
    let mut list = SqList::from(&TEST_DATA[..]);
    assert!(list.set(0, 10));
    assert_eq!(list[0], 10);
    assert!(!list.set(99, 10));
    assert_eq!(list.get(99), None);
}

#[test]
fn test_find() {
    let list = SqList::from(&TEST_DATA[..]);
    assert_eq!(list.find(3), Some(3));
    assert_eq!(list.find(99), None);
}

#[test]
fn test_insert() {
    let mut list = SqList::from(&TEST_DATA[..]);
    assert!(list.insert(0, 10));
    assert_eq!(list[0], 10);
    assert_eq!(list.len(), 7);

    assert!(list.insert(6, 10));
    assert_eq!(list[6], 10);
    assert_eq!(list.len(), 8);

    assert!(list.insert(3, 10));
    assert_eq!(list[3], 10);
    assert_eq!(list.len(), 9);

    assert!(!list.insert(99, 10));
    assert_eq!(list.len(), 9);
}

#[test]
fn test_push_front() {
    let mut list = SqList::from(&TEST_DATA[..]);
    assert!(list.push_front(10));
    assert_eq!(list[0], 10);
    assert_eq!(list.len(), 7);
}

#[test]
fn test_push_back() {
    let mut list = SqList::from(&TEST_DATA[..]);
    assert!(list.push_back(10));
    assert_eq!(list[6], 10);
    assert_eq!(list.len(), 7);
}

#[test]
fn test_remove() {
    let mut list = SqList::from(&TEST_DATA[..]);
    assert_eq!(list.remove(0), Some(0));
    assert_eq!(list.len(), 5);
    assert_eq!(list[0..list.len()], [1, 2, 3, 4, 5]);

    assert_eq!(list.remove(4), Some(5));
    assert_eq!(list.len(), 4);
    assert_eq!(list[0..list.len()], [1, 2, 3, 4]);

    assert_eq!(list.remove(99), None);
    assert_eq!(list.len(), 4);

    list.clear();
    assert_eq!(list.remove(0), None);
}

#[test]
fn test_pop_front() {
    let mut list = SqList::from(&TEST_DATA[..]);
    assert_eq!(list.pop_front(), Some(0));
    assert_eq!(list.len(), 5);
    assert_eq!(list[0..list.len()], [1, 2, 3, 4, 5]);

    list.clear();
    assert_eq!(list.pop_front(), None);
}

#[test]
fn test_pop_back() {
    let mut list = SqList::from(&TEST_DATA[..]);
    assert_eq!(list.pop_back(), Some(5));
    assert_eq!(list.len(), 5);
    assert_eq!(list[0..list.len()], [0, 1, 2, 3, 4]);

    list.clear();
    assert_eq!(list.pop_back(), None);
}
