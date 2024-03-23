use crate::ds::Node;
use std::fmt::{self, Debug, Display};
use std::mem;
use std::ops::{Index, IndexMut};
use std::{marker::PhantomData, ptr::NonNull};

#[derive(Debug, Clone)]
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

impl<T: Debug> Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cursor = self.cursor_front();
        write!(f, "[")?;
        while let Some(node) = cursor.current {
            unsafe {
                write!(f, "{:?}", node.as_ref().val)?;
                cursor.move_next();
                if cursor.current.is_some() {
                    write!(f, " <-> ")?;
                }
            }
        }
        write!(f, "]")
    }
}

impl<T: Copy + Default> From<&[T]> for LinkedList<T> {
    fn from(value: &[T]) -> Self {
        let mut list = Self::new();
        for &e in value {
            list.push_back(e);
        }
        list
    }
}

impl<T: Copy + Default> From<&LinkedList<T>> for Vec<T> {
    fn from(list: &LinkedList<T>) -> Self {
        let mut v = Vec::with_capacity(list.len);
        let mut cursor = list.cursor_front();

        while let Some(&val) = cursor.current() {
            v.push(val);
            cursor.move_next();
        }
        v
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }

        let mut cursor1 = self.cursor_front();
        let mut cursor2 = other.cursor_front();

        while let (Some(v1), Some(v2)) = (cursor1.current(), cursor2.current()) {
            if v1 != v2 {
                return false;
            }
            cursor1.move_next();
            cursor2.move_next();
        }

        true
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

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> Index<usize> for LinkedList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.iter().nth(index).unwrap()
    }
}

impl<T> IndexMut<usize> for LinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.iter_mut().nth(index).unwrap()
    }
}

impl<T> LinkedList<T> {
    unsafe fn unlink(&mut self, mut node: NonNull<Node<T>>) {
        let node = node.as_mut();

        match node.prev {
            Some(prev) => (*prev.as_ptr()).next = node.next,
            None => self.head = node.next,
        }

        match node.next {
            Some(next) => (*next.as_ptr()).prev = node.prev,
            None => self.tail = node.prev,
        }

        self.len -= 1;
    }

    unsafe fn push_front_node(&mut self, node: NonNull<Node<T>>) {
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

    unsafe fn push_back_node(&mut self, node: NonNull<Node<T>>) {
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

    pub fn contains(&self, val: &T) -> bool
    where
        T: PartialEq,
    {
        self.iter().any(|v| v == val)
    }

    pub fn front(&self) -> Option<&T> {
        self.head
            .as_ref()
            .map(|node| unsafe { &(*node.as_ptr()).val })
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head
            .as_mut()
            .map(|node| unsafe { &mut (*node.as_ptr()).val })
    }

    pub fn back(&self) -> Option<&T> {
        self.tail
            .as_ref()
            .map(|node| unsafe { &(*node.as_ptr()).val })
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.tail
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

    pub fn insert(&mut self, at: usize, val: T) -> bool {
        if at > self.len() {
            return false;
        } else if at == self.len() {
            self.push_back(val);
        } else if at == 0 {
            self.push_front(val);
        } else {
            let offset = self.len() - at - 1;
            if at < offset {
                let mut cursor = self.cursor_front_mut();
                for _ in 0..at {
                    cursor.move_next();
                }
                cursor.insert_before(val);
            } else {
                let mut cursor = self.cursor_back_mut();
                for _ in 0..offset {
                    cursor.move_prev();
                }
                cursor.insert_before(val);
            }
        }
        true
    }

    pub fn remove(&mut self, at: usize) -> Option<T> {
        if at >= self.len() {
            None
        } else {
            let offset = self.len() - at - 1;
            if at < offset {
                let mut cursor = self.cursor_front_mut();
                for _ in 0..at {
                    cursor.move_next();
                }
                cursor.remove_current()
            } else {
                let mut cursor = self.cursor_back_mut();
                for _ in 0..offset {
                    cursor.move_prev();
                }
                cursor.remove_current()
            }
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len,
            marker: PhantomData,
        }
    }

    pub fn cursor_front(&self) -> Cursor<'_, T> {
        Cursor {
            index: 0,
            current: self.head,
            list: self,
        }
    }

    pub fn cursor_front_mut(&mut self) -> CursorMut<'_, T> {
        CursorMut {
            index: 0,
            current: self.head,
            list: self,
        }
    }

    pub fn cursor_back(&self) -> Cursor<'_, T> {
        Cursor {
            index: self.len().checked_sub(1).unwrap_or(0),
            current: self.tail,
            list: self,
        }
    }

    pub fn cursor_back_mut(&mut self) -> CursorMut<'_, T> {
        CursorMut {
            index: self.len().checked_sub(1).unwrap_or(0),
            current: self.tail,
            list: self,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Iter<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    #[allow(unused)]
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<T> Default for Iter<'_, T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
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
            self.head.map(|node| unsafe {
                let node = &*node.as_ptr();
                self.head = node.next;
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
            self.tail.map(|node| unsafe {
                let node = &*node.as_ptr();
                self.len -= 1;
                self.tail = node.prev;
                &node.val
            })
        }
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    #[allow(unused)]
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a mut Node<T>>,
}

impl<T> Default for IterMut<'_, T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
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
            self.head.map(|node| unsafe {
                let node = &mut *node.as_ptr();
                self.head = node.next;
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
            self.tail.map(|node| unsafe {
                let node = &mut *node.as_ptr();
                self.len -= 1;
                self.tail = node.prev;
                &mut node.val
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.list.pop_back()
    }
}

#[derive(Debug, Clone)]
pub struct Cursor<'a, T: 'a> {
    index: usize,
    current: Option<NonNull<Node<T>>>,
    list: &'a LinkedList<T>,
}

impl<'a, T> Cursor<'a, T> {
    pub fn index(&self) -> Option<usize> {
        let _ = self.current?;
        Some(self.index)
    }

    pub fn move_next(&mut self) {
        match self.current.take() {
            Some(cur) => unsafe {
                self.current = cur.as_ref().next;
                self.index += 1;
            },
            None => {
                self.index = 0;
                self.current = self.list.head;
            }
        }
    }

    pub fn move_prev(&mut self) {
        match self.current.take() {
            Some(cur) => unsafe {
                self.current = cur.as_ref().prev;
                self.index.checked_sub(1).unwrap_or(0);
            },
            None => {
                self.current = self.list.tail;
                self.index.checked_div(1).unwrap_or(self.list.len());
            }
        }
    }

    pub fn current(&self) -> Option<&'a T> {
        unsafe { self.current.map(|cur| &(*cur.as_ptr()).val) }
    }

    pub fn peek_next(&self) -> Option<&'a T> {
        unsafe {
            let next = match self.current {
                Some(cur) => cur.as_ref().next,
                None => self.list.head,
            };
            next.map(|node| &(*node.as_ptr()).val)
        }
    }

    pub fn peek_prev(&self) -> Option<&'a T> {
        unsafe {
            let prev = match self.current {
                Some(cur) => cur.as_ref().prev,
                None => self.list.tail,
            };
            prev.map(|node| &(*node.as_ptr()).val)
        }
    }

    pub fn front(&self) -> Option<&'a T> {
        self.list.front()
    }

    pub fn back(&self) -> Option<&'a T> {
        self.list.back()
    }
}

#[derive(Debug)]
pub struct CursorMut<'a, T: 'a> {
    index: usize,
    current: Option<NonNull<Node<T>>>,
    list: &'a mut LinkedList<T>,
}

impl<'a, T> CursorMut<'a, T> {
    pub fn index(&self) -> Option<usize> {
        let _ = self.current?;
        Some(self.index)
    }

    pub fn move_next(&mut self) {
        match self.current.take() {
            Some(cur) => unsafe {
                self.current = cur.as_ref().next;
                self.index += 1;
            },
            None => {
                self.index = 0;
                self.current = self.list.head;
            }
        }
    }

    pub fn move_prev(&mut self) {
        match self.current.take() {
            Some(cur) => unsafe {
                self.current = cur.as_ref().prev;
                self.index.checked_sub(1).unwrap_or(0);
            },
            None => {
                self.current = self.list.tail;
                self.index.checked_div(1).unwrap_or(self.list.len());
            }
        }
    }

    pub fn current(&self) -> Option<&'a mut T> {
        unsafe { self.current.map(|cur| &mut (*cur.as_ptr()).val) }
    }

    pub fn front(&self) -> Option<&T> {
        self.list.front()
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.list.front_mut()
    }

    pub fn back(&self) -> Option<&T> {
        self.list.back()
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.list.back_mut()
    }

    pub fn peek_next(&mut self) -> Option<&'a mut T> {
        unsafe {
            let next = match self.current {
                Some(cur) => cur.as_ref().next,
                None => self.list.head,
            };
            next.map(|node| &mut (*node.as_ptr()).val)
        }
    }

    pub fn peek_prev(&mut self) -> Option<&'a mut T> {
        unsafe {
            let prev = match self.current {
                Some(cur) => cur.as_ref().prev,
                None => self.list.tail,
            };
            prev.map(|node| &mut (*node.as_ptr()).val)
        }
    }

    pub fn push_front(&mut self, val: T) {
        self.list.push_front(val);
        self.index += 1;
    }

    pub fn push_back(&mut self, val: T) {
        self.list.push_back(val);
        if self.current.is_none() {
            self.index += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.list.is_empty() {
            None
        } else {
            if self.current == self.list.head {
                self.move_next();
            } else {
                self.index -= 1;
            }
            self.list.pop_front()
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.list.is_empty() {
            None
        } else {
            if self.current == self.list.tail {
                self.current = None;
            } else if self.current.is_none() {
                self.index = self.list.len() - 1;
            }
            self.list.pop_back()
        }
    }

    pub fn insert_before(&mut self, val: T) {
        let node = Box::new(Node::new(val));
        let node_ptr = NonNull::from(Box::leak(node));

        match self.current {
            Some(cur) => unsafe {
                (*node_ptr.as_ptr()).next = self.current;
                (*node_ptr.as_ptr()).prev = cur.as_ref().prev;
                if self.current == self.list.head {
                    self.list.head = Some(node_ptr);
                } else {
                    (*cur.as_ref().prev.unwrap().as_ptr()).next = Some(node_ptr);
                }
                (*cur.as_ptr()).prev = Some(node_ptr);
            },
            None => unsafe {
                self.list.push_back_node(node_ptr);
            },
        }
        self.index += 1;
    }

    pub fn insert_after(&mut self, val: T) {
        let node = Box::new(Node::new(val));
        let node_ptr = NonNull::from(Box::leak(node));

        match self.current {
            Some(cur) => unsafe {
                (*node_ptr.as_ptr()).next = cur.as_ref().next;
                (*node_ptr.as_ptr()).prev = self.current;
                if self.current == self.list.tail {
                    self.list.tail = Some(node_ptr);
                } else {
                    (*cur.as_ref().next.unwrap().as_ptr()).prev = Some(node_ptr);
                }
                (*cur.as_ptr()).next = Some(node_ptr);
            },
            None => unsafe {
                self.list.push_back_node(node_ptr);
            },
        }
        self.index += 1;
    }

    pub fn remove_current(&mut self) -> Option<T> {
        let unlinked_node = self.current?;
        unsafe {
            self.current = unlinked_node.as_ref().next;
            self.list.unlink(unlinked_node);
            let unlinked_node = Box::from_raw(unlinked_node.as_ptr());
            Some(unlinked_node.val)
        }
    }
}
