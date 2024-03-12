use crate::ds::Node;
use std::io::Lines;
use std::mem;
use std::{marker::PhantomData, ptr::NonNull};

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut LinkedList<T>);
        impl<'a, T> Drop for DropGuard<'a, T> {
            fn drop(&mut self) {
                while self.0.pop_front_node().is_some() {}
            }
        }

        let guard = DropGuard(self);
        while guard.0.pop_front_node().is_some() {}
        mem::forget(guard);
    }
}

impl<T> LinkedList<T> {
    fn push_front_node(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            (*node.as_ptr()).prev = None;
            (*node.as_ptr()).next = self.head;
            let node = Some(node);

            match self.head {
                Some(head) => (*head.as_ptr()).prev = node,
                None => self.tail = node,
            }

            self.head = node;
            self.len += 1;
        }
    }

    fn push_back_node(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            (*node.as_ptr()).prev = self.tail;
            (*node.as_ptr()).next = None;
            let node = Some(node);

            match self.tail {
                Some(tail) => (*tail.as_ptr()).next = node,
                None => self.head = node,
            }

            self.tail = node;
            self.len += 1;
        }
    }

    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;

            match self.head {
                Some(head) => (*head.as_ptr()).prev = None,
                None => self.tail = None,
            }

            self.len -= 1;
            node
        })
    }

    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;

            match self.tail {
                Some(tail) => (*tail.as_ptr()).next = None,
                None => self.head = None,
            }

            self.len -= 1;
            node
        })
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        drop(LinkedList {
            head: self.head.take(),
            tail: self.tail.take(),
            len: mem::take(&mut self.len),
            marker: PhantomData,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
