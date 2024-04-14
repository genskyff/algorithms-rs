use crate::ds::MAXLEN;
use std::cmp;
use std::fmt::Display;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
struct SNode<T> {
    data: T,
    prev: Option<usize>,
    next: Option<usize>,
}

impl<T: Copy + Default> Default for SNode<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            prev: None,
            next: None,
        }
    }
}

// node[0] is the head
// node[len - 1] is the tail
#[derive(Debug, Clone)]
pub struct SLinkedList<T> {
    nodes: [SNode<T>; MAXLEN],
    space: Option<usize>,
    head: Option<usize>,
    tail: Option<usize>,
    len: usize,
}

// trait impls

impl<T: Copy + Default> Default for SLinkedList<T> {
    fn default() -> Self {
        let mut nodes = [SNode::default(); MAXLEN];

        for i in 0..MAXLEN {
            nodes[i].next = Some(i + 1);
            nodes[i].prev = None;
        }
        nodes[MAXLEN - 1].next = None;

        Self {
            nodes,
            space: Some(0),
            head: None,
            tail: None,
            len: 0,
        }
    }
}

impl<T: Copy + Default, const N: usize> From<[T; N]> for SLinkedList<T> {
    fn from(arr: [T; N]) -> Self {
        Self::from_slice(&arr)
    }
}

impl<T: Copy + Default> From<&[T]> for SLinkedList<T> {
    fn from(s: &[T]) -> Self {
        Self::from_slice(s)
    }
}

impl<T: Copy + Default> From<&SLinkedList<T>> for Vec<T> {
    fn from(list: &SLinkedList<T>) -> Self {
        let mut v = Vec::with_capacity(list.len);
        let mut idx = list.head;

        while let Some(i) = idx {
            v.push(list.nodes[i].data);
            idx = list.nodes[i].next;
        }

        v
    }
}

impl<T: PartialEq> PartialEq for SLinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().eq(other.iter())
    }
}

impl<T: PartialEq, const N: usize> PartialEq<[T; N]> for SLinkedList<T> {
    fn eq(&self, other: &[T; N]) -> bool {
        self.len == N && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T: PartialEq> PartialEq<&[T]> for SLinkedList<T> {
    fn eq(&self, other: &&[T]) -> bool {
        self.len == other.len() && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T: PartialEq> PartialEq<Vec<T>> for SLinkedList<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        self.len == other.len() && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T: Display> Display for SLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut idx = self.head;
        write!(f, "[")?;
        while let Some(i) = idx {
            write!(f, "{}", self.nodes[i].data)?;
            idx = self.nodes[i].next;
            if idx.is_some() {
                write!(f, " <-> ")?;
            }
        }
        write!(f, "]")
    }
}

impl<T> Index<usize> for SLinkedList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.iter().nth(index).unwrap()
    }
}

impl<T> IndexMut<usize> for SLinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.iter_mut().nth(index).unwrap()
    }
}

// private impls

impl<T> SLinkedList<T> {
    fn alloc(&mut self) -> Option<usize> {
        match self.space {
            Some(idx) => {
                self.space = self.nodes[idx].next;
                Some(idx)
            }
            None => None,
        }
    }

    fn free(&mut self, idx: usize) {
        self.nodes[idx].next = self.space;
        self.space = Some(idx);
    }

    fn from_slice(arr: &[T]) -> Self
    where
        T: Copy + Default,
    {
        let mut list = Self::new();

        for i in 0..cmp::min(arr.len(), MAXLEN) {
            if let Some(idx) = list.alloc() {
                list.nodes[idx].data = arr[i];
                list.nodes[idx].next = None;

                if list.head.is_none() {
                    list.nodes[idx].prev = None;
                    list.head = Some(idx);
                } else {
                    list.nodes[idx].prev = list.tail;
                    list.nodes[list.tail.unwrap()].next = Some(idx);
                }

                list.tail = Some(idx);
                list.len += 1;
            } else {
                break;
            }
        }

        list
    }
}

// public impls

impl<T> SLinkedList<T> {
    pub fn new() -> Self
    where
        T: Copy + Default,
    {
        Default::default()
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
        let mut idx = self.head;
        while let Some(i) = idx {
            idx = self.nodes[i].next;
            self.free(i);
        }

        self.space = Some(0);
        self.head = None;
        self.tail = None;
        self.len = 0;

        for i in 0..MAXLEN {
            self.nodes[i].next = Some(i + 1);
            self.nodes[i].prev = None;
        }

        self.nodes[MAXLEN - 1].next = None;
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn swap(&mut self, mut i: usize, mut j: usize) {
        if self.is_empty() || i == j || cmp::max(i, j) >= self.len {
            return;
        }

        if i > j {
            std::mem::swap(&mut i, &mut j);
        }

        let mut node_i = 0;
        let mut node_j = 0;
        let mut cur = self.head;
        for k in 0..=j {
            if k == i {
                node_i = cur.unwrap();
            }
            if k == j {
                node_j = cur.unwrap();
            }
            cur = self.nodes[cur.unwrap()].next;
        }

        let i_prev = self.nodes[node_i].prev;
        let i_next = self.nodes[node_i].next;
        let j_prev = self.nodes[node_j].prev;
        let j_next = self.nodes[node_j].next;

        // adjacent nodes
        if i_next == Some(node_j) {
            // node_i is directly connected to node_j
            self.nodes[node_i].next = j_next;
            if j_next.is_some() {
                self.nodes[j_next.unwrap()].prev = Some(node_i);
            }

            self.nodes[node_j].prev = i_prev;
            if i_prev.is_some() {
                self.nodes[i_prev.unwrap()].next = Some(node_j);
            }

            self.nodes[node_j].next = Some(node_i);
            self.nodes[node_i].prev = Some(node_j);
        } else {
            // non-adjacent nodes
            if i_prev.is_some() {
                self.nodes[i_prev.unwrap()].next = Some(node_j);
            }

            if i_next.is_some() {
                self.nodes[i_next.unwrap()].prev = Some(node_j);
            }

            if j_prev.is_some() {
                self.nodes[j_prev.unwrap()].next = Some(node_i);
            }

            if j_next.is_some() {
                self.nodes[j_next.unwrap()].prev = Some(node_i);
            }

            self.nodes[node_i].prev = j_prev;
            self.nodes[node_j].prev = i_prev;

            let tmp = self.nodes[node_i].next;
            self.nodes[node_i].next = self.nodes[node_j].next;
            self.nodes[node_j].next = tmp;
        }

        if self.head == Some(node_i) {
            self.head = Some(node_j);
        } else if self.head == Some(node_j) {
            self.head = Some(node_i);
        }

        if self.tail == Some(node_i) {
            self.tail = Some(node_j);
        } else if self.tail == Some(node_j) {
            self.tail = Some(node_i);
        }
    }

    pub fn reverse(&mut self) {
        if !self.is_empty() {
            let mut i = 0;
            let mut j = self.len - 1;
            while i < j {
                self.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
    }

    pub fn front(&self) -> Option<&T> {
        self.iter().next()
    }

    pub fn back(&self) -> Option<&T> {
        self.iter().last()
    }

    pub fn contains(&self, elem: &T) -> bool
    where
        T: PartialEq,
    {
        self.iter().any(|v| v == elem)
    }

    pub fn find(&self, elem: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        self.iter().position(|v| v == elem)
    }

    pub fn find_all(&self, elem: &T) -> Vec<usize>
    where
        T: PartialEq,
    {
        self.iter()
            .enumerate()
            .filter_map(|(i, v)| (v == elem).then(|| i))
            .collect()
    }

    pub fn insert(&mut self, at: usize, elem: T) -> bool {
        if at > self.len {
            return false;
        }

        if let Some(idx) = self.alloc() {
            self.nodes[idx].data = elem;

            if at == 0 {
                self.nodes[idx].prev = None;
                self.nodes[idx].next = self.head;
                if self.head.is_some() {
                    self.nodes[self.head.unwrap()].prev = Some(idx);
                }
                self.head = Some(idx);
            } else if at == self.len {
                self.nodes[idx].prev = self.tail;
                self.nodes[idx].next = None;
                if self.tail.is_some() {
                    self.nodes[self.tail.unwrap()].next = Some(idx);
                }
                self.tail = Some(idx);
            } else {
                let mut cur = self.head.unwrap();
                for _ in 0..at {
                    cur = self.nodes[cur].next.unwrap();
                }
                self.nodes[idx].prev = self.nodes[cur].prev;
                self.nodes[idx].next = Some(cur);
                self.nodes[cur].prev = Some(idx);
                if self.nodes[idx].prev.is_some() {
                    self.nodes[self.nodes[idx].prev.unwrap()].next = Some(idx);
                }
            }

            self.len += 1;
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, at: usize) -> Option<T>
    where
        T: Copy,
    {
        if at >= self.len {
            return None;
        }

        let mut cur = self.head.unwrap();
        for _ in 0..at {
            cur = self.nodes[cur].next.unwrap();
        }

        let e = self.nodes[cur].data;
        if at == 0 {
            self.head = self.nodes[cur].next;
            if self.head.is_some() {
                self.nodes[self.head.unwrap()].prev = None;
            }
        } else if at == self.len - 1 {
            self.tail = self.nodes[cur].prev;
            if self.tail.is_some() {
                self.nodes[self.tail.unwrap()].next = None;
            }
        } else {
            if self.nodes[cur].prev.is_some() {
                self.nodes[self.nodes[cur].prev.unwrap()].next = self.nodes[cur].next;
            }
            if self.nodes[cur].next.is_some() {
                self.nodes[self.nodes[cur].next.unwrap()].prev = self.nodes[cur].prev;
            }
        }

        self.free(cur);
        self.len -= 1;
        Some(e)
    }

    pub fn push_front(&mut self, elem: T) -> bool {
        self.insert(0, elem)
    }

    pub fn push_back(&mut self, elem: T) -> bool {
        self.insert(self.len, elem)
    }

    pub fn pop_front(&mut self) -> Option<T>
    where
        T: Copy,
    {
        self.remove(0)
    }

    pub fn pop_back(&mut self) -> Option<T>
    where
        T: Copy,
    {
        self.remove(self.len.checked_sub(1).unwrap_or(0))
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head,
            tail: self.tail,
            list: self,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            list: self,
        }
    }
}

// iterator impls

#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    list: SLinkedList<T>,
}

impl<T: Copy> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T: Copy> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.list.pop_back()
    }
}

impl<T: Copy> IntoIterator for SLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

#[derive(Debug, Clone)]
pub struct Iter<'a, T: 'a> {
    head: Option<usize>,
    tail: Option<usize>,
    list: &'a SLinkedList<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.take().map(|i| {
            self.head = self.list.nodes[i].next;
            &self.list.nodes[i].data
        })
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.tail.take().map(|i| {
            self.tail = self.list.nodes[i].prev;
            &self.list.nodes[i].data
        })
    }
}

impl<'a, T> IntoIterator for &'a SLinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T: 'a> {
    head: Option<usize>,
    tail: Option<usize>,
    list: &'a mut SLinkedList<T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.take().map(|i| {
            self.head = self.list.nodes[i].next;
            unsafe { &mut (*self.list.nodes.as_mut_ptr().add(i)).data }
        })
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.tail.take().map(|i| {
            self.tail = self.list.nodes[i].prev;
            unsafe { &mut (*self.list.nodes.as_mut_ptr().add(i)).data }
        })
    }
}

impl<'a, T> IntoIterator for &'a mut SLinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
