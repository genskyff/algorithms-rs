use crate::ds::Node;
use std::fmt::Display;
use std::mem;
use std::{marker::PhantomData, ptr::NonNull};

#[derive(Debug, Clone)]
pub struct LinkedStack<T> {
    top: Option<NonNull<Node<T>>>,
    bottom: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

// trait impls

impl<T> Default for LinkedStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy + Default, const N: usize> From<[T; N]> for LinkedStack<T> {
    fn from(arr: [T; N]) -> Self {
        Self::from_slice(&arr)
    }
}

impl<T: Copy + Default> From<&[T]> for LinkedStack<T> {
    fn from(s: &[T]) -> Self {
        Self::from_slice(s)
    }
}

impl<T: Copy + Default> From<&LinkedStack<T>> for Vec<T> {
    fn from(stack: &LinkedStack<T>) -> Self {
        let mut v = Vec::with_capacity(stack.len);
        stack.into_iter().rev().for_each(|&e| v.push(e));
        v
    }
}

impl<T: PartialEq> PartialEq for LinkedStack<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T: PartialEq, const N: usize> PartialEq<[T; N]> for LinkedStack<T> {
    fn eq(&self, other: &[T; N]) -> bool {
        self.len == N && self.iter().zip(other.iter().rev()).all(|(a, b)| a == b)
    }
}

impl<T: PartialEq> PartialEq<&[T]> for LinkedStack<T> {
    fn eq(&self, other: &&[T]) -> bool {
        self.len == other.len() && self.iter().zip(other.iter().rev()).all(|(a, b)| a == b)
    }
}

impl<T: PartialEq> PartialEq<Vec<T>> for LinkedStack<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        self.len == other.len() && self.iter().zip(other.iter().rev()).all(|(a, b)| a == b)
    }
}

impl<T> Drop for LinkedStack<T> {
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut LinkedStack<T>);
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

impl<T: Display> Display for LinkedStack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.into_iter().peekable();
        write!(f, "[")?;
        while let Some(v) = iter.next_back() {
            write!(f, "{}", v)?;
            if iter.peek().is_some() {
                write!(f, " -> ")?;
            }
        }
        write!(f, "]")
    }
}

// private impls

impl<T> LinkedStack<T> {
    unsafe fn push_back_node(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            (*node.as_ptr()).prev = self.top;
            (*node.as_ptr()).next = None;
            let node = Some(node);

            match self.top {
                Some(top) => (*top.as_ptr()).next = node,
                None => self.bottom = node,
            }

            self.top = node;
            self.len += 1;
        }
    }

    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.bottom.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.bottom = node.next;

            match self.bottom {
                Some(bottom) => (*bottom.as_ptr()).prev = None,
                None => self.top = None,
            }

            self.len -= 1;
            node
        })
    }

    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        self.top.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.top = node.prev;

            match self.top {
                Some(top) => (*top.as_ptr()).next = None,
                None => self.bottom = None,
            }

            self.len -= 1;
            node
        })
    }

    fn pop_bottom(&mut self) -> Option<T> {
        self.pop_front_node().map(|node| node.into_val())
    }

    fn from_slice(slice: &[T]) -> Self
    where
        T: Copy,
    {
        let mut stack = Self::new();
        for &e in slice {
            stack.push(e);
        }
        stack
    }
}

// public impls

impl<T> LinkedStack<T> {
    pub fn new() -> Self {
        Self {
            top: None,
            bottom: None,
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
        drop(LinkedStack {
            top: self.top.take(),
            bottom: self.bottom.take(),
            len: mem::take(&mut self.len),
            marker: PhantomData,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.top.is_none()
    }

    pub fn peek(&self) -> Option<&T> {
        self.top
            .as_ref()
            .map(|node| unsafe { &(*node.as_ptr()).val })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.top
            .as_mut()
            .map(|node| unsafe { &mut (*node.as_ptr()).val })
    }

    pub fn push(&mut self, val: T) {
        let node = Box::new(Node::new(val));
        let node_ptr = NonNull::from(Box::leak(node));
        unsafe {
            self.push_back_node(node_ptr);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_back_node().map(|node| node.into_val())
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            top: self.top,
            bottom: self.bottom,
            len: self.len,
            marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            top: self.top,
            bottom: self.bottom,
            len: self.len,
            marker: PhantomData,
        }
    }
}

// iterator impls

#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    stack: LinkedStack<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.stack.pop_bottom()
    }
}

impl<T> IntoIterator for LinkedStack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { stack: self }
    }
}

#[derive(Debug, Clone)]
pub struct Iter<'a, T: 'a> {
    top: Option<NonNull<Node<T>>>,
    bottom: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<T> Default for Iter<'_, T> {
    fn default() -> Self {
        Self {
            top: None,
            bottom: None,
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
            self.top.map(|node| unsafe {
                let node = &*node.as_ptr();
                self.top = node.prev;
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
            self.bottom.map(|node| unsafe {
                let node = &*node.as_ptr();
                self.len -= 1;
                self.bottom = node.next;
                &node.val
            })
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedStack<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T: 'a> {
    top: Option<NonNull<Node<T>>>,
    bottom: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a mut Node<T>>,
}

impl<T> Default for IterMut<'_, T> {
    fn default() -> Self {
        Self {
            top: None,
            bottom: None,
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
            self.top.map(|node| unsafe {
                let node = &mut *node.as_ptr();
                self.top = node.prev;
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
            self.bottom.map(|node| unsafe {
                let node = &mut *node.as_ptr();
                self.len -= 1;
                self.bottom = node.next;
                &mut node.val
            })
        }
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedStack<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
