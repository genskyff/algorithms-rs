use algorithms_rs::ds::SLinkedList;

const LEN: usize = 6;
const TEST_DATA: [i32; LEN] = [0, 1, 2, 3, 4, 5];

// test trait impls

#[test]
fn test_default() {
    let list = SLinkedList::<i32>::default();
    assert_eq!(list.len(), 0);
    assert_eq!(list.front(), None);
    assert_eq!(list.back(), None);
}

#[test]
fn test_from() {
    let list = SLinkedList::from(TEST_DATA);
    assert_eq!(list.len(), LEN);
    assert_eq!(list, TEST_DATA);
}

#[test]
fn test_eq() {
    let list1 = SLinkedList::from(TEST_DATA);
    let list2 = SLinkedList::from(TEST_DATA);
    assert_eq!(list1, list2);
    assert_eq!(list1, TEST_DATA);
    assert_eq!(list1, &TEST_DATA[..]);
    assert_eq!(list1, TEST_DATA.to_vec());
    assert_ne!(list1, &TEST_DATA[1..]);
}

#[test]
fn test_display() {
    let mut list = SLinkedList::from(TEST_DATA);
    assert_eq!(format!("{}", list), "[0 <-> 1 <-> 2 <-> 3 <-> 4 <-> 5]");
    list.clear();
    assert_eq!(format!("{}", list), "[]");
}

#[test]
fn test_index() {
    let list = SLinkedList::from(TEST_DATA);
    assert_eq!(list[0], 0);
}

#[test]
fn test_index_mut() {
    let mut list = SLinkedList::from(TEST_DATA);
    list[0] = 10;
    assert_eq!(list[0], 10);
}

#[test]
fn test_into_iter() {
    let list = SLinkedList::from(TEST_DATA);
    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next_back(), Some(5));
}

#[test]
fn test_iter() {
    let list = SLinkedList::from(TEST_DATA);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next_back(), Some(&5));
}

#[test]
fn test_iter_mut() {
    let mut list = SLinkedList::from(TEST_DATA);
    for e in list.iter_mut() {
        *e += 1;
    }
    assert_eq!(list, vec![1, 2, 3, 4, 5, 6]);
}

// test impls

#[test]
fn test_new() {
    let list = SLinkedList::<i32>::new();
    assert_eq!(list.len(), 0);
    assert_eq!(list.front(), None);
    assert_eq!(list.back(), None);
}

#[test]
fn test_to_vec() {
    let list = SLinkedList::from(TEST_DATA);
    assert_eq!(list.to_vec(), TEST_DATA.to_vec());
}

#[test]
fn test_clear() {
    let mut list = SLinkedList::from(TEST_DATA);
    list.clear();
    assert_eq!(list.len(), 0);
    assert_eq!(list, vec![]);
}

#[test]
fn test_is_empty() {
    let mut list = SLinkedList::from(TEST_DATA);
    assert!(!list.is_empty());
    list.clear();
    assert!(list.is_empty());
}

#[test]
fn test_swap() {
    let mut list = SLinkedList::from(TEST_DATA);
    list.swap(0, 5);
    assert_eq!(list, vec![5, 1, 2, 3, 4, 0]);

    list.swap(0, 99);
    assert_eq!(list, vec![5, 1, 2, 3, 4, 0]);
}

#[test]
fn test_reverse() {
    let mut list = SLinkedList::from(TEST_DATA);
    list.reverse();
    assert_eq!(list, vec![5, 4, 3, 2, 1, 0]);
}

#[test]
fn test_front() {
    let mut list = SLinkedList::from(TEST_DATA);
    assert_eq!(list.front(), Some(&0));
    list.clear();
    assert_eq!(list.front(), None);
}

#[test]
fn test_back() {
    let mut list = SLinkedList::from(TEST_DATA);
    assert_eq!(list.back(), Some(&5));
    list.clear();
    assert_eq!(list.back(), None);
}

#[test]
fn test_find() {
    let list = SLinkedList::from(TEST_DATA);
    assert_eq!(list.find(&3), Some(3));
    assert_eq!(list.find(&99), None);
}

#[test]
fn test_find_all() {
    let list = SLinkedList::from(TEST_DATA);
    assert_eq!(list.find_all(&3), vec![3]);
    assert_eq!(list.find_all(&99), vec![]);
}

#[test]
fn test_insert() {
    let mut list = SLinkedList::from(TEST_DATA);
    assert!(list.insert(0, 10));
    assert_eq!(list, vec![10, 0, 1, 2, 3, 4, 5]);
    assert!(list.insert(list.len(), 10));
    assert_eq!(list, vec![10, 0, 1, 2, 3, 4, 5, 10]);
    assert!(!list.insert(99, 10));
    assert_eq!(list, vec![10, 0, 1, 2, 3, 4, 5, 10]);
}

#[test]
fn test_remove() {
    let mut list = SLinkedList::from(TEST_DATA);
    assert_eq!(list.remove(0), Some(0));
    assert_eq!(list, &TEST_DATA[1..]);

    assert_eq!(list.remove(4), Some(5));
    assert_eq!(list, &TEST_DATA[1..5]);

    assert_eq!(list.remove(99), None);
    assert_eq!(list, &TEST_DATA[1..5]);

    list.clear();
    assert_eq!(list.remove(0), None);
}

#[test]
fn test_push_front() {
    let mut list = SLinkedList::from(TEST_DATA);
    list.push_front(10);
    assert_eq!(list, vec![10, 0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_push_back() {
    let mut list = SLinkedList::from(TEST_DATA);
    list.push_back(10);
    assert_eq!(list, vec![0, 1, 2, 3, 4, 5, 10]);
}

#[test]
fn test_pop_front() {
    let mut list = SLinkedList::from(TEST_DATA);
    assert_eq!(list.pop_front(), Some(0));
    assert_eq!(list, &TEST_DATA[1..]);
}

#[test]
fn test_pop_back() {
    let mut list = SLinkedList::from(TEST_DATA);
    assert_eq!(list.pop_back(), Some(5));
    assert_eq!(list, &TEST_DATA[..5]);
}
