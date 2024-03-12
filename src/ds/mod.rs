const MAXLEN: usize = 100;

pub(super) mod node {
    use std::ptr::NonNull;

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
    }
}

/// # Linked List
pub mod linked_list;
pub use linked_list::LinkedList;

/// # Sequential List
pub mod sqlist;
pub use sqlist::SqList;

/// # Static linked list
pub mod static_linked_list;
pub use static_linked_list::SLinkedList;
