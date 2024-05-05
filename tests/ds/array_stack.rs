use algorithms_rs::ds::ArrayStack;

const LEN: usize = 6;
const TEST_DATA: [i32; LEN] = [0, 1, 2, 3, 4, 5];

// test trait impls

#[test]
fn test_default() {
    let stack = ArrayStack::<i32>::default();
    assert_eq!(stack.len(), 0);
}

#[test]
fn test_from() {
    let stack = ArrayStack::from(TEST_DATA);
    assert_eq!(stack.len(), LEN);
    assert_eq!(stack, TEST_DATA);
}

#[test]
fn test_eq() {
    let stack1 = ArrayStack::from(TEST_DATA);
    let stack2 = ArrayStack::from(TEST_DATA);
    assert_eq!(stack1, stack2);
    assert_eq!(stack1, TEST_DATA);
    assert_eq!(stack1, &TEST_DATA[..]);
    assert_eq!(stack1, TEST_DATA.to_vec());
    assert_ne!(stack1, &TEST_DATA[1..]);
}

#[test]
fn test_display() {
    let mut stack = ArrayStack::from(TEST_DATA);
    assert_eq!(format!("{}", stack), "[0, 1, 2, 3, 4, 5]");
    stack.clear();
    assert_eq!(format!("{}", stack), "[]");
}

// test impls

#[test]
fn test_new() {
    let stack = ArrayStack::<i32>::new();
    assert_eq!(stack.len(), 0);
}

#[test]
fn test_clear() {
    let mut stack = ArrayStack::from(TEST_DATA);
    stack.clear();
    assert_eq!(stack.len(), 0);
    assert_eq!(stack, []);
}

#[test]
fn peek() {
    let mut stack = ArrayStack::from(TEST_DATA);
    assert_eq!(stack.peek(), Some(&5));
    stack.clear();
    assert_eq!(stack.peek(), None);
}

#[test]
fn peek_mut() {
    let mut stack = ArrayStack::from(TEST_DATA);
    stack.peek_mut().map(|e| *e = 10);
    assert_eq!(stack.peek(), Some(&10));
    stack.clear();
    assert_eq!(stack.peek_mut(), None);
}

#[test]
fn test_push() {
    let mut stack = ArrayStack::from(TEST_DATA);
    assert!(stack.push(10));
    assert_eq!(stack[6], 10);
    assert_eq!(stack.len(), LEN + 1);
}

#[test]
fn test_pop() {
    let mut stack = ArrayStack::from(TEST_DATA);
    assert_eq!(stack.pop(), Some(5));
    assert_eq!(stack.len(), LEN - 1);
    assert_eq!(stack, &TEST_DATA[..LEN - 1]);
    stack.clear();
    assert_eq!(stack.pop(), None);
}
