const MAXLEN: usize = 100;
pub use node::Node;

mod node {
    use std::ptr::NonNull;

    #[derive(Debug, Clone)]
    pub struct Node<T> {
        pub val: T,
        pub prev: Option<NonNull<Node<T>>>,
        pub next: Option<NonNull<Node<T>>>,
    }

    impl<T> Node<T> {
        pub fn new(val: T) -> Self {
            Self {
                val,
                prev: None,
                next: None,
            }
        }

        pub fn into_val(self: Box<Self>) -> T {
            self.val
        }
    }
}

/// # Array Queue
pub mod array_queue;
pub use array_queue::ArrayQueue;

/// # Array Stack
pub mod array_stack;
pub use array_stack::ArrayStack;

/// # HashMap
pub mod hashmap;
pub use hashmap::HashMap;

/// # Linked List
pub mod linked_list;
pub use linked_list::LinkedList;

/// # Linked Queue
pub mod linked_queue;
pub use linked_queue::LinkedQueue;

/// # Linked Stack
pub mod linked_stack;
pub use linked_stack::LinkedStack;

/// # Sequential List
pub mod sqlist;
pub use sqlist::SqList;

/// # Static linked list
pub mod static_linked_list;
pub use static_linked_list::SLinkedList;

/// # Vector
pub mod vector;
pub use vector::Vector;
