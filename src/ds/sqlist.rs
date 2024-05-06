use crate::ds::MAXLEN;
use std::cmp;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

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

impl<T: Copy + Default, const N: usize> From<[T; N]> for SqList<T> {
    fn from(arr: [T; N]) -> Self {
        Self::from_slice(&arr)
    }
}

impl<T: Copy + Default> From<&[T]> for SqList<T> {
    fn from(s: &[T]) -> Self {
        Self::from_slice(s)
    }
}

impl<T: PartialEq> PartialEq for SqList<T> {
    fn eq(&self, other: &Self) -> bool {
        self[..] == other[..]
    }
}

impl<T: PartialEq, const N: usize> PartialEq<[T; N]> for SqList<T> {
    fn eq(&self, other: &[T; N]) -> bool {
        &self[..] == other
    }
}

impl<T: PartialEq> PartialEq<&[T]> for SqList<T> {
    fn eq(&self, other: &&[T]) -> bool {
        &self[..] == *other
    }
}

impl<T: PartialEq> PartialEq<Vec<T>> for SqList<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        &self[..] == other.as_slice()
    }
}

impl<T: Display> Display for SqList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, e) in self.iter().enumerate() {
            write!(f, "{}{}", e, if i == self.len - 1 { "" } else { ", " })?;
        }
        write!(f, "]")
    }
}

impl<T> Deref for SqList<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data[0..self.len]
    }
}

impl<T> DerefMut for SqList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data[0..self.len]
    }
}

// private impls

impl<T> SqList<T> {
    fn from_slice(slice: &[T]) -> Self
    where
        T: Copy + Default,
    {
        let mut list = Self::default();
        let len = cmp::min(slice.len(), MAXLEN);
        list.data[..len].copy_from_slice(&slice[..len]);
        list.len = len;
        list
    }
}

// public impls

impl<T> SqList<T> {
    pub fn new() -> Self
    where
        T: Copy + Default,
    {
        Default::default()
    }

    pub fn clear(&mut self) {
        self.len = 0;
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
            .filter_map(|(i, v)| (v == elem).then_some(i))
            .collect()
    }

    pub fn insert(&mut self, at: usize, elem: T) -> bool {
        if self.len == MAXLEN || at > self.len {
            return false;
        }

        if at < self.len {
            self.data[at..].rotate_right(1);
        }
        self.data[at] = elem;
        self.len += 1;

        true
    }

    pub fn remove(&mut self, at: usize) -> Option<T>
    where
        T: Copy,
    {
        if at >= self.len {
            return None;
        }

        let deleted = self.data[at];
        if at < self.len - 1 {
            self.data[at..].rotate_left(1);
        }
        self.len -= 1;

        Some(deleted)
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
        self.remove(self.len.saturating_sub(1))
    }
}
