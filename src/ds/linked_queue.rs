use crate::ds::Node;
use std::fmt::Display;
use std::mem;
use std::{marker::PhantomData, ptr::NonNull};

#[derive(Debug, Clone)]
pub struct LinkedQueue<T> {
    front: Option<NonNull<Node<T>>>,
    rear: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

// trait impls

impl<T> Default for LinkedQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy + Default, const N: usize> From<[T; N]> for LinkedQueue<T> {
    fn from(arr: [T; N]) -> Self {
        Self::from_slice(&arr)
    }
}

impl<T: Copy + Default> From<&[T]> for LinkedQueue<T> {
    fn from(s: &[T]) -> Self {
        Self::from_slice(s)
    }
}

impl<T: Copy + Default> From<&LinkedQueue<T>> for Vec<T> {
    fn from(queue: &LinkedQueue<T>) -> Self {
        let mut v = Vec::with_capacity(queue.len);
        queue.into_iter().for_each(|&e| v.push(e));
        v
    }
}

impl<T: PartialEq> PartialEq for LinkedQueue<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T: PartialEq, const N: usize> PartialEq<[T; N]> for LinkedQueue<T> {
    fn eq(&self, other: &[T; N]) -> bool {
        self.len == N && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T: PartialEq> PartialEq<&[T]> for LinkedQueue<T> {
    fn eq(&self, other: &&[T]) -> bool {
        self.len == other.len() && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T: PartialEq> PartialEq<Vec<T>> for LinkedQueue<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        self.len == other.len() && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T> Drop for LinkedQueue<T> {
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut LinkedQueue<T>);
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

impl<T: Display> Display for LinkedQueue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.into_iter().peekable();
        write!(f, "[")?;
        while let Some(v) = iter.next() {
            write!(f, "{}", v)?;
            if iter.peek().is_some() {
                write!(f, " <-> ")?;
            }
        }
        write!(f, "]")
    }
}

// private impls

impl<T> LinkedQueue<T> {
    unsafe fn push_front_node(&mut self, node: NonNull<Node<T>>) {
        (*node.as_ptr()).prev = None;
        (*node.as_ptr()).next = self.front;
        let node = Some(node);

        match self.front {
            Some(front) => (*front.as_ptr()).prev = node,
            None => self.rear = node,
        }

        self.front = node;
        self.len += 1;
    }

    unsafe fn push_back_node(&mut self, node: NonNull<Node<T>>) {
        (*node.as_ptr()).prev = self.rear;
        (*node.as_ptr()).next = None;
        let node = Some(node);

        match self.rear {
            Some(rear) => (*rear.as_ptr()).next = node,
            None => self.front = node,
        }

        self.rear = node;
        self.len += 1;
    }

    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.front.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.front = node.next;

            match self.front {
                Some(front) => (*front.as_ptr()).prev = None,
                None => self.rear = None,
            }

            self.len -= 1;
            node
        })
    }

    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        self.rear.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.rear = node.prev;

            match self.rear {
                Some(rear) => (*rear.as_ptr()).next = None,
                None => self.front = None,
            }

            self.len -= 1;
            node
        })
    }

    fn from_slice(slice: &[T]) -> Self
    where
        T: Copy,
    {
        let mut queue = Self::new();
        for &e in slice {
            queue.push_back(e);
        }
        queue
    }
}

// public impls

impl<T> LinkedQueue<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            rear: None,
            len: 0,
            marker: PhantomData,
        }
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Copy + Default,
    {
        Vec::from(self)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        drop(LinkedQueue {
            front: self.front.take(),
            rear: self.rear.take(),
            len: mem::take(&mut self.len),
            marker: PhantomData,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.front.is_none()
    }

    pub fn front(&self) -> Option<&T> {
        self.front
            .as_ref()
            .map(|node| unsafe { &(*node.as_ptr()).val })
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.front
            .as_mut()
            .map(|node| unsafe { &mut (*node.as_ptr()).val })
    }

    pub fn back(&self) -> Option<&T> {
        self.rear
            .as_ref()
            .map(|node| unsafe { &(*node.as_ptr()).val })
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.rear
            .as_mut()
            .map(|node| unsafe { &mut (*node.as_ptr()).val })
    }

    pub fn push_front(&mut self, val: T) {
        let node = Box::new(Node::new(val));
        let node_ptr = NonNull::from(Box::leak(node));
        unsafe {
            self.push_front_node(node_ptr);
        }
    }

    pub fn push_back(&mut self, val: T) {
        let node = Box::new(Node::new(val));
        let node_ptr = NonNull::from(Box::leak(node));
        unsafe {
            self.push_back_node(node_ptr);
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(|node| node.into_val())
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(|node| node.into_val())
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            front: self.front,
            rear: self.rear,
            len: self.len,
            marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            front: self.front,
            rear: self.rear,
            len: self.len,
            marker: PhantomData,
        }
    }
}

// iterator impls

#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    queue: LinkedQueue<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.queue.pop_back()
    }
}

impl<T> IntoIterator for LinkedQueue<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { queue: self }
    }
}

#[derive(Debug, Clone)]
pub struct Iter<'a, T: 'a> {
    front: Option<NonNull<Node<T>>>,
    #[allow(unused)]
    rear: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<T> Default for Iter<'_, T> {
    fn default() -> Self {
        Self {
            front: None,
            rear: None,
            len: 0,
            marker: Default::default(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.front.map(|node| unsafe {
                let node = &*node.as_ptr();
                self.front = node.next;
                self.len -= 1;
                &node.val
            })
        }
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            None
        } else {
            self.rear.map(|node| unsafe {
                let node = &*node.as_ptr();
                self.len -= 1;
                self.rear = node.prev;
                &node.val
            })
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedQueue<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T: 'a> {
    front: Option<NonNull<Node<T>>>,
    #[allow(unused)]
    rear: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a mut Node<T>>,
}

impl<T> Default for IterMut<'_, T> {
    fn default() -> Self {
        Self {
            front: None,
            rear: None,
            len: 0,
            marker: Default::default(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.front.map(|node| unsafe {
                let node = &mut *node.as_ptr();
                self.front = node.next;
                self.len -= 1;
                &mut node.val
            })
        }
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut T> {
        if self.len == 0 {
            None
        } else {
            self.rear.map(|node| unsafe {
                let node = &mut *node.as_ptr();
                self.len -= 1;
                self.rear = node.prev;
                &mut node.val
            })
        }
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedQueue<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
