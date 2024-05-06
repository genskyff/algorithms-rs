use crate::ds::MAXLEN;
use std::cmp;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ArrayStack<T> {
    data: [T; MAXLEN],
    len: usize,
}

// trait impls

impl<T: Copy + Default> Default for ArrayStack<T> {
    fn default() -> Self {
        Self {
            data: [Default::default(); MAXLEN],
            len: 0,
        }
    }
}

impl<T: Copy + Default, const N: usize> From<[T; N]> for ArrayStack<T> {
    fn from(arr: [T; N]) -> Self {
        Self::from_slice(&arr)
    }
}

impl<T: Copy + Default> From<&[T]> for ArrayStack<T> {
    fn from(s: &[T]) -> Self {
        Self::from_slice(s)
    }
}

impl<T: PartialEq> PartialEq for ArrayStack<T> {
    fn eq(&self, other: &Self) -> bool {
        self[..] == other[..]
    }
}

impl<T: PartialEq, const N: usize> PartialEq<[T; N]> for ArrayStack<T> {
    fn eq(&self, other: &[T; N]) -> bool {
        &self[..] == other
    }
}

impl<T: PartialEq> PartialEq<&[T]> for ArrayStack<T> {
    fn eq(&self, other: &&[T]) -> bool {
        &self[..] == *other
    }
}

impl<T: PartialEq> PartialEq<Vec<T>> for ArrayStack<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        &self[..] == other.as_slice()
    }
}

impl<T: Display> Display for ArrayStack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, e) in self.iter().enumerate() {
            write!(f, "{}{}", e, if i == self.len - 1 { "" } else { ", " })?;
        }
        write!(f, "]")
    }
}

impl<T> Deref for ArrayStack<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data[0..self.len]
    }
}

impl<T> DerefMut for ArrayStack<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data[0..self.len]
    }
}

// private impls

impl<T> ArrayStack<T> {
    fn from_slice(slice: &[T]) -> Self
    where
        T: Copy + Default,
    {
        let mut stack = Self::default();
        let len = cmp::min(slice.len(), MAXLEN);
        stack.data[..len].copy_from_slice(&slice[..len]);
        stack.len = len;
        stack
    }
}

// public impls

impl<T> ArrayStack<T> {
    pub fn new() -> Self
    where
        T: Copy + Default,
    {
        Default::default()
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn peek(&self) -> Option<&T> {
        match self.len.checked_sub(1) {
            Some(i) => self.data.get(i),
            None => None,
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        match self.len.checked_sub(1) {
            Some(i) => self.data.get_mut(i),
            None => None,
        }
    }

    pub fn push(&mut self, elem: T) -> bool {
        if self.len == MAXLEN {
            return false;
        }

        self.data[self.len] = elem;
        self.len += 1;
        true
    }

    pub fn pop(&mut self) -> Option<T>
    where
        T: Copy,
    {
        if self.is_empty() {
            return None;
        }

        self.len -= 1;
        Some(self.data[self.len])
    }
}
