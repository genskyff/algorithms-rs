use algorithms_rs::ds::LinkedQueue;

const LEN: usize = 6;
const TEST_DATA: [i32; 6] = [0, 1, 2, 3, 4, 5];

// test trait impls

#[test]
fn test_default() {
    let queue = LinkedQueue::<i32>::default();
    assert_eq!(queue.len(), 0);
    assert_eq!(queue.front(), None);
    assert_eq!(queue.back(), None);
}

#[test]
fn test_from() {
    let queue = LinkedQueue::from(TEST_DATA);
    assert_eq!(queue.len(), LEN);
    assert_eq!(queue, TEST_DATA);
}

#[test]
fn test_eq() {
    let queue1 = LinkedQueue::from(TEST_DATA);
    let queue2 = LinkedQueue::from(TEST_DATA);
    assert_eq!(queue1, queue2);
    assert_eq!(queue1, TEST_DATA);
    assert_eq!(queue1, &TEST_DATA[..]);
    assert_eq!(queue1, TEST_DATA.to_vec());
    assert_ne!(queue1, &TEST_DATA[1..]);
}

#[test]
fn test_display() {
    let mut queue = LinkedQueue::from(TEST_DATA);
    assert_eq!(format!("{}", queue), "[0 <-> 1 <-> 2 <-> 3 <-> 4 <-> 5]");
    queue.clear();
    assert_eq!(format!("{}", queue), "[]");
}

#[test]
fn test_into_iter() {
    let queue = LinkedQueue::from(TEST_DATA);
    let mut iter = queue.into_iter();
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next_back(), Some(5));
}

#[test]
fn test_iter() {
    let queue = LinkedQueue::from(TEST_DATA);
    let mut iter = queue.iter();
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next_back(), Some(&5));
}

#[test]
fn test_iter_mut() {
    let mut queue = LinkedQueue::from(TEST_DATA);
    for e in queue.iter_mut() {
        *e += 1;
    }
    assert_eq!(queue, vec![1, 2, 3, 4, 5, 6]);
}

// test impls

#[test]
fn test_new() {
    let queue = LinkedQueue::<i32>::new();
    assert_eq!(queue.len(), 0);
    assert_eq!(queue.front(), None);
    assert_eq!(queue.back(), None);
}

#[test]
fn test_to_vec() {
    let queue = LinkedQueue::from(TEST_DATA);
    assert_eq!(queue.to_vec(), TEST_DATA.to_vec());
}

#[test]
fn test_clear() {
    let mut queue = LinkedQueue::from(TEST_DATA);
    queue.clear();
    assert_eq!(queue.len(), 0);
    assert_eq!(queue, vec![]);
}

#[test]
fn test_is_empty() {
    let mut queue = LinkedQueue::from(TEST_DATA);
    assert!(!queue.is_empty());
    queue.clear();
    assert!(queue.is_empty());
}

#[test]
fn test_front() {
    let mut queue = LinkedQueue::from(TEST_DATA);
    assert_eq!(queue.front(), Some(&0));
    queue.clear();
    assert_eq!(queue.front(), None);
}

#[test]
fn test_front_mut() {
    let mut queue = LinkedQueue::from(TEST_DATA);
    *queue.front_mut().unwrap() = 10;
    assert_eq!(queue.front(), Some(&10));
    queue.clear();
    assert_eq!(queue.front_mut(), None);
}

#[test]
fn test_back() {
    let mut queue = LinkedQueue::from(TEST_DATA);
    assert_eq!(queue.back(), Some(&5));
    queue.clear();
    assert_eq!(queue.back(), None);
}

#[test]
fn test_back_mut() {
    let mut queue = LinkedQueue::from(TEST_DATA);
    *queue.back_mut().unwrap() = 10;
    assert_eq!(queue.back(), Some(&10));
    queue.clear();
    assert_eq!(queue.back_mut(), None);
}

#[test]
fn test_push_front() {
    let mut queue = LinkedQueue::from(TEST_DATA);
    queue.push_front(10);
    assert_eq!(queue, vec![10, 0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_push_back() {
    let mut queue = LinkedQueue::from(TEST_DATA);
    queue.push_back(10);
    assert_eq!(queue, vec![0, 1, 2, 3, 4, 5, 10]);
}

#[test]
fn test_pop_front() {
    let mut queue = LinkedQueue::from(TEST_DATA);
    assert_eq!(queue.pop_front(), Some(0));
    assert_eq!(queue, &TEST_DATA[1..]);
}

#[test]
fn test_pop_back() {
    let mut queue = LinkedQueue::from(TEST_DATA);
    assert_eq!(queue.pop_back(), Some(5));
    assert_eq!(queue, &TEST_DATA[..5]);
}
