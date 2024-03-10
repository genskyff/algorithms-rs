use crate::ds::MAXLEN;
use std::cmp;
use std::fmt::{self, Debug, Display};
use std::ops::{Index, IndexMut, Range};

#[derive(Debug, Clone, Copy)]
struct SNode<T> {
    data: T,
    prev: Option<usize>,
    next: Option<usize>,
}

// node[0] is the head
// node[len - 1] is the tail
#[derive(Debug)]
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
}
