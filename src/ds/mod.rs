const MAXLEN: usize = 100;
pub use node::Node;

mod node {
    use std::collections::LinkedList;
    use std::ptr::NonNull;

    #[derive(Debug)]
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

/// # Linked List
pub mod linked_list;
pub use linked_list::LinkedList;

/// # Sequential List
pub mod sqlist;
pub use sqlist::SqList;

/// # Static linked list
pub mod static_linked_list;
pub use static_linked_list::SLinkedList;
