use crate::ds::MAXLEN;
use std::cmp;
use std::fmt::{self, Debug, Display};
use std::ops::{Index, IndexMut, Range};

#[derive(Debug)]
pub struct SqList<T> {
    data: [T; MAXLEN],
    len: usize,
}

// trait impls

impl<T: Copy + Default> Default for SqList<T> {
    fn default() -> Self {
        Self {
            data: [Default::default(); MAXLEN],
            len: 0,
        }
    }
}

impl<T: Debug> Display for SqList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self.data[0..self.len])
    }
}

impl<T: Copy + Default> From<&[T]> for SqList<T> {
    fn from(arr: &[T]) -> Self {
        let mut list = Self::default();
        for i in 0..cmp::min(arr.len(), MAXLEN) {
            list.data[i] = arr[i];
        }
        list.len = arr.len();
        list
    }
}

impl<T: PartialEq> PartialEq for SqList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }

        for i in 0..self.len {
            if self.data[i] != other.data[i] {
                return false;
            }
        }
        true
    }
}

impl<T> AsRef<[T]> for SqList<T> {
    fn as_ref(&self) -> &[T] {
        &self.data[0..self.len]
    }
}

impl<T> AsMut<[T]> for SqList<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.data[0..self.len]
    }
}

impl<T> Index<usize> for SqList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for SqList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Index<Range<usize>> for SqList<T> {
    type Output = [T];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.data[range]
    }
}

impl<T> IndexMut<Range<usize>> for SqList<T> {
    fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
        &mut self.data[range]
    }
}

// impls

impl<T: Copy + Default> SqList<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        if cmp::max(i, j) < self.len {
            self.data.swap(i, j);
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
        self.len = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, i: usize) -> Option<T> {
        if i < self.len {
            Some(self.data[i])
        } else {
            None
        }
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
            self.data[i] = e;
            true
        } else {
            false
        }
    }

    pub fn find(&self, e: T) -> Option<usize>
    where
        T: PartialEq,
    {
        for i in 0..self.len {
            if self.data[i] == e {
                return Some(i);
            }
        }
        None
    }

    pub fn insert(&mut self, i: usize, e: T) -> bool {
        if self.len == MAXLEN || i > self.len {
            return false;
        }

        if i < self.len {
            self.data[i..self.len].rotate_right(1);
        }

        self.data[i] = e;
        self.len += 1;
        true
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

        let e = self.data[i];
        if i < self.len - 1 {
            self.data[i..self.len].rotate_left(1);
        }
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
