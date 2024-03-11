use crate::ds::MAXLEN;
use std::cmp;
use std::fmt::{self, Debug, Display};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
struct SNode<T> {
    data: T,
    prev: Option<usize>,
    next: Option<usize>,
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

impl<T: Copy + Default> Default for SNode<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            prev: None,
            next: None,
        }
    }
}

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

impl<T> Display for SLinkedList<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut idx = self.head;
        write!(f, "[")?;
        while let Some(i) = idx {
            write!(f, "{:?}", self.nodes[i].data)?;
            idx = self.nodes[i].next;
            if idx.is_some() {
                write!(f, " <-> ")?;
            }
        }
        write!(f, "]")
    }
}

impl<T: Copy + Default> From<&[T]> for SLinkedList<T> {
    fn from(arr: &[T]) -> Self {
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

impl<T: Copy + Default> From<SLinkedList<T>> for Vec<T> {
    fn from(list: SLinkedList<T>) -> Self {
        let mut v = Vec::with_capacity(list.len);
        let mut idx = list.head;

        while let Some(i) = idx {
            v.push(list.nodes[i].data);
            idx = list.nodes[i].next;
        }

        v
    }
}

impl<T> PartialEq for SLinkedList<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }

        let mut idx1 = self.head;
        let mut idx2 = other.head;

        while let (Some(i1), Some(i2)) = (idx1, idx2) {
            if self.nodes[i1].data != other.nodes[i2].data {
                return false;
            }
            idx1 = self.nodes[i1].next;
            idx2 = other.nodes[i2].next;
        }

        true
    }
}

impl<T: Copy + Default> IntoIterator for SLinkedList<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::from(self).into_iter()
    }
}

impl<T> Index<usize> for SLinkedList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut idx = self.head;
        let mut i = 0;

        while let Some(j) = idx {
            if i == index {
                return &self.nodes[j].data;
            }
            idx = self.nodes[j].next;
            i += 1;
        }

        panic!("Index out of bounds");
    }
}

impl<T> IndexMut<usize> for SLinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut idx = self.head;
        let mut i = 0;

        while let Some(j) = idx {
            if i == index {
                return &mut self.nodes[j].data;
            }
            idx = self.nodes[j].next;
            i += 1;
        }

        panic!("Index out of bounds");
    }
}

// impls

impl<T: Copy + Default> SLinkedList<T> {
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
}

impl<T: Copy + Default> SLinkedList<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn to_vec(&self) -> Vec<T> {
        Vec::from(self.clone())
    }

    pub fn len(&self) -> usize {
        self.len
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

    pub fn get(&self, i: usize) -> Option<T> {
        if i < self.len {
            if i < (self.len + 1) / 2 {
                let mut idx = self.head;
                for _ in 0..i {
                    idx = self.nodes[idx.unwrap()].next;
                }
                return Some(self.nodes[idx.unwrap()].data);
            } else {
                let mut idx = self.tail;
                for _ in 0..(self.len - i - 1) {
                    idx = self.nodes[idx.unwrap()].prev;
                }
                return Some(self.nodes[idx.unwrap()].data);
            }
        }

        None
    }

    pub fn first(&self) -> Option<T> {
        self.get(0)
    }

    pub fn last(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.get(self.len - 1)
        }
    }

    pub fn set(&mut self, i: usize, e: T) -> bool {
        if i < self.len {
            if i < (self.len + 1) / 2 {
                let mut idx = self.head;
                for _ in 0..i {
                    idx = self.nodes[idx.unwrap()].next;
                }
                self.nodes[idx.unwrap()].data = e;
            } else {
                let mut idx = self.tail;
                for _ in 0..(self.len - i - 1) {
                    idx = self.nodes[idx.unwrap()].prev;
                }
                self.nodes[idx.unwrap()].data = e;
            }
            true
        } else {
            false
        }
    }

    pub fn find(&self, e: T) -> Option<usize>
    where
        T: PartialEq,
    {
        let mut idx = self.head;
        let mut i = 0;
        while let Some(j) = idx {
            if self.nodes[j].data == e {
                return Some(i);
            }
            idx = self.nodes[j].next;
            i += 1;
        }
        None
    }

    pub fn insert(&mut self, i: usize, e: T) -> bool {
        if i > self.len {
            return false;
        }

        if let Some(idx) = self.alloc() {
            self.nodes[idx].data = e;

            if i == 0 {
                self.nodes[idx].prev = None;
                self.nodes[idx].next = self.head;
                if self.head.is_some() {
                    self.nodes[self.head.unwrap()].prev = Some(idx);
                }
                self.head = Some(idx);
            } else if i == self.len {
                self.nodes[idx].prev = self.tail;
                self.nodes[idx].next = None;
                if self.tail.is_some() {
                    self.nodes[self.tail.unwrap()].next = Some(idx);
                }
                self.tail = Some(idx);
            } else {
                let mut cur = self.head.unwrap();
                for _ in 0..i {
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

    pub fn push_front(&mut self, e: T) -> bool {
        self.insert(0, e)
    }

    pub fn push_back(&mut self, e: T) -> bool {
        self.insert(self.len, e)
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        if i >= self.len {
            return None;
        }

        let mut cur = self.head.unwrap();
        for _ in 0..i {
            cur = self.nodes[cur].next.unwrap();
        }

        let e = self.nodes[cur].data;
        if i == 0 {
            self.head = self.nodes[cur].next;
            if self.head.is_some() {
                self.nodes[self.head.unwrap()].prev = None;
            }
        } else if i == self.len - 1 {
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

    pub fn pop_front(&mut self) -> Option<T> {
        self.remove(0)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.remove(self.len - 1)
        }
    }
}
