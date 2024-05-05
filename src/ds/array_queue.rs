use crate::ds::MAXLEN;
use std::cmp;
use std::fmt::Display;

#[derive(Debug)]
pub struct ArrayQueue<T> {
    data: [T; MAXLEN],
    front: usize,
    len: usize,
}

// trait impls

impl<T: Copy + Default> Default for ArrayQueue<T> {
    fn default() -> Self {
        Self {
            data: [Default::default(); MAXLEN],
            front: 0,
            len: 0,
        }
    }
}

impl<T: Copy + Default, const N: usize> From<[T; N]> for ArrayQueue<T> {
    fn from(arr: [T; N]) -> Self {
        Self::from_slice(&arr)
    }
}

impl<T: Copy + Default> From<&[T]> for ArrayQueue<T> {
    fn from(s: &[T]) -> Self {
        Self::from_slice(s)
    }
}

impl<T: PartialEq> PartialEq for ArrayQueue<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data
            .iter()
            .cycle()
            .skip(self.front)
            .take(self.len)
            .eq(other.data.iter().cycle().skip(other.front).take(other.len))
    }
}

impl<T: PartialEq, const N: usize> PartialEq<[T; N]> for ArrayQueue<T> {
    fn eq(&self, other: &[T; N]) -> bool {
        self.data
            .iter()
            .cycle()
            .skip(self.front)
            .take(self.len)
            .eq(other.iter())
    }
}

impl<T: PartialEq> PartialEq<&[T]> for ArrayQueue<T> {
    fn eq(&self, other: &&[T]) -> bool {
        self.data
            .iter()
            .cycle()
            .skip(self.front)
            .take(self.len)
            .eq(other.iter())
    }
}

impl<T: PartialEq> PartialEq<Vec<T>> for ArrayQueue<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        self.data
            .iter()
            .cycle()
            .skip(self.front)
            .take(self.len)
            .eq(other.iter())
    }
}

impl<T: Display> Display for ArrayQueue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, e) in self
            .data
            .iter()
            .cycle()
            .skip(self.front)
            .take(self.len)
            .enumerate()
        {
            write!(f, "{}{}", e, if i == self.len - 1 { "" } else { ", " })?;
        }
        write!(f, "]")
    }
}

// private impls

impl<T> ArrayQueue<T> {
    fn from_slice(slice: &[T]) -> Self
    where
        T: Copy + Default,
    {
        let mut queue = Self::default();
        let len = cmp::min(slice.len(), MAXLEN);
        for i in 0..len {
            queue.data[i] = slice[i];
        }
        queue.len = len;
        queue
    }
}

// public impls

impl<T> ArrayQueue<T> {
    pub fn new() -> Self
    where
        T: Copy + Default,
    {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        self.front = 0;
        self.len = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        Some(&self.data[self.front])
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }

        Some(&mut self.data[self.front])
    }

    pub fn back(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        let back = (self.front + self.len - 1) % MAXLEN;
        Some(&self.data[back])
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }

        let back = (self.front + self.len - 1) % MAXLEN;
        Some(&mut self.data[back])
    }

    pub fn push_front(&mut self, elem: T) -> bool {
        if self.len == MAXLEN {
            return false;
        }

        self.front = (self.front.checked_sub(1).unwrap_or(MAXLEN - 1)) % MAXLEN;
        self.data[self.front] = elem;
        self.len += 1;

        true
    }

    pub fn push_back(&mut self, elem: T) -> bool {
        if self.len == MAXLEN {
            return false;
        }

        let back = (self.front + self.len) % MAXLEN;
        self.data[back] = elem;
        self.len += 1;

        true
    }

    pub fn pop_front(&mut self) -> Option<T>
    where
        T: Copy,
    {
        if self.is_empty() {
            return None;
        }

        let deleted = self.data[self.front];
        self.front = (self.front + 1) % MAXLEN;
        self.len -= 1;

        Some(deleted)
    }

    pub fn pop_back(&mut self) -> Option<T>
    where
        T: Copy,
    {
        if self.is_empty() {
            return None;
        }

        let back = (self.front + self.len - 1) % MAXLEN;
        let deleted = self.data[back];
        self.len -= 1;

        Some(deleted)
    }
}
