use algorithms_rs::ds::LinkedStack;

const LEN: usize = 6;
const TEST_DATA: [i32; 6] = [0, 1, 2, 3, 4, 5];

// test trait impls

#[test]
fn test_default() {
    let stack = LinkedStack::<i32>::default();
    assert_eq!(stack.len(), 0);
    assert_eq!(stack.peek(), None);
}

#[test]
fn test_from() {
    let stack = LinkedStack::from(TEST_DATA);
    assert_eq!(stack.len(), LEN);
    assert_eq!(stack, TEST_DATA);
}

#[test]
fn test_eq() {
    let stack1 = LinkedStack::from(TEST_DATA);
    let stack2 = LinkedStack::from(TEST_DATA);
    assert_eq!(stack1, stack2);
    assert_eq!(stack1, TEST_DATA);
    assert_eq!(stack1, &TEST_DATA[..]);
    assert_eq!(stack1, TEST_DATA.to_vec());
    assert_ne!(stack1, &TEST_DATA[1..]);
}

#[test]
fn test_display() {
    let mut stack = LinkedStack::from(TEST_DATA);
    assert_eq!(format!("{}", stack), "[0 -> 1 -> 2 -> 3 -> 4 -> 5]");
    stack.clear();
    assert_eq!(format!("{}", stack), "[]");
}

#[test]
fn test_into_iter() {
    let stack = LinkedStack::from(TEST_DATA);
    let mut iter = stack.into_iter();
    assert_eq!(iter.next(), Some(5));
}

#[test]
fn test_iter() {
    let stack = LinkedStack::from(TEST_DATA);
    let mut iter = stack.iter();
    assert_eq!(iter.next(), Some(&5));
}

#[test]
fn test_iter_mut() {
    let mut stack = LinkedStack::from(TEST_DATA);
    for e in stack.iter_mut() {
        *e += 1;
    }
    assert_eq!(stack, vec![1, 2, 3, 4, 5, 6]);
}

// test impls

#[test]
fn test_new() {
    let stack = LinkedStack::<i32>::new();
    assert_eq!(stack.len(), 0);
    assert_eq!(stack.peek(), None);
}

#[test]
fn test_to_vec() {
    let stack = LinkedStack::from(TEST_DATA);
    assert_eq!(stack.to_vec(), TEST_DATA.to_vec());
}

#[test]
fn test_clear() {
    let mut stack = LinkedStack::from(TEST_DATA);
    stack.clear();
    assert_eq!(stack.len(), 0);
    assert_eq!(stack, vec![]);
}

#[test]
fn test_is_empty() {
    let mut stack = LinkedStack::from(TEST_DATA);
    assert!(!stack.is_empty());
    stack.clear();
    assert!(stack.is_empty());
}

#[test]
fn test_peek() {
    let mut stack = LinkedStack::from(TEST_DATA);
    assert_eq!(stack.peek(), Some(&5));
    stack.clear();
    assert_eq!(stack.peek(), None);
}

#[test]
fn test_peek_mut() {
    let mut stack = LinkedStack::from(TEST_DATA);
    *stack.peek_mut().unwrap() = 10;
    assert_eq!(stack.peek(), Some(&10));
    stack.clear();
    assert_eq!(stack.peek_mut(), None);
}

#[test]
fn test_push() {
    let mut stack = LinkedStack::from(TEST_DATA);
    stack.push(10);
    assert_eq!(stack, vec![0, 1, 2, 3, 4, 5, 10]);
}

#[test]
fn test_pop() {
    let mut stack = LinkedStack::from(TEST_DATA);
    assert_eq!(stack.pop(), Some(5));
    assert_eq!(stack, &TEST_DATA[..5]);
}
