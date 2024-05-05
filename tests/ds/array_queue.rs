use algorithms_rs::ds::ArrayQueue;

const LEN: usize = 6;
const TEST_DATA: [i32; LEN] = [0, 1, 2, 3, 4, 5];

// test trait impls

#[test]
fn test_default() {
    let queue = ArrayQueue::<i32>::default();
    assert_eq!(queue.len(), 0);
}

#[test]
fn test_from() {
    let queue = ArrayQueue::from(TEST_DATA);
    assert_eq!(queue.len(), LEN);
    assert_eq!(queue, TEST_DATA);
}

#[test]
fn test_eq() {
    let queue1 = ArrayQueue::from(TEST_DATA);
    let queue2 = ArrayQueue::from(TEST_DATA);
    assert_eq!(queue1, queue2);
    assert_eq!(queue1, TEST_DATA);
    assert_eq!(queue1, &TEST_DATA[..]);
    assert_eq!(queue1, TEST_DATA.to_vec());
    assert_ne!(queue1, &TEST_DATA[1..]);
}

#[test]
fn test_display() {
    let mut queue = ArrayQueue::from(TEST_DATA);
    assert_eq!(format!("{}", queue), "[0, 1, 2, 3, 4, 5]");
    queue.clear();
    assert_eq!(format!("{}", queue), "[]");
}

// test impls

#[test]
fn test_new() {
    let queue = ArrayQueue::<i32>::new();
    assert_eq!(queue.len(), 0);
}

#[test]
fn test_len() {
    let queue = ArrayQueue::from(TEST_DATA);
    assert_eq!(queue.len(), LEN);
}

#[test]
fn test_clear() {
    let mut queue = ArrayQueue::from(TEST_DATA);
    queue.clear();
    assert_eq!(queue.len(), 0);
    assert_eq!(queue, []);
}

#[test]
fn test_is_empty() {
    let queue = ArrayQueue::<i32>::new();
    assert!(queue.is_empty());
}

#[test]
fn test_front() {
    let mut queue = ArrayQueue::from(TEST_DATA);
    assert_eq!(queue.front(), Some(&0));
    queue.clear();
    assert_eq!(queue.front(), None);
}

#[test]
fn test_front_mut() {
    let mut queue = ArrayQueue::from(TEST_DATA);
    queue.front_mut().map(|e| *e = 10);
    assert_eq!(queue.front(), Some(&10));
}

#[test]
fn test_back() {
    let mut queue = ArrayQueue::from(TEST_DATA);
    assert_eq!(queue.back(), Some(&5));
    queue.clear();
    assert_eq!(queue.back(), None);
}

#[test]
fn test_back_mut() {
    let mut queue = ArrayQueue::from(TEST_DATA);
    queue.back_mut().map(|e| *e = 10);
    assert_eq!(queue.back(), Some(&10));
}

#[test]
fn test_push_front() {
    let mut queue = ArrayQueue::from(TEST_DATA);
    assert!(queue.push_front(10));
    assert_eq!(queue.front(), Some(&10));
    assert_eq!(queue.len(), LEN + 1);
}

#[test]
fn test_push_back() {
    let mut queue = ArrayQueue::from(TEST_DATA);
    assert!(queue.push_back(10));
    assert_eq!(queue.back(), Some(&10));
    assert_eq!(queue.len(), LEN + 1);
}

#[test]
fn test_pop_front() {
    let mut queue = ArrayQueue::from(TEST_DATA);
    assert_eq!(queue.pop_front(), Some(0));
    assert_eq!(queue.len(), LEN - 1);
    assert_eq!(queue, &TEST_DATA[1..]);

    queue.clear();
    assert_eq!(queue.pop_front(), None);
}

#[test]
fn test_pop_back() {
    let mut queue = ArrayQueue::from(TEST_DATA);
    assert_eq!(queue.pop_back(), Some(5));
    assert_eq!(queue.len(), LEN - 1);
    assert_eq!(queue, &TEST_DATA[..LEN - 1]);

    queue.clear();
    assert_eq!(queue.pop_back(), None);
}
