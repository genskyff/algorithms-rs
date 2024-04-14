use std::alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout};
use std::fmt::Display;
use std::mem::size_of;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

#[derive(Debug)]
struct RawVector<T> {
    ptr: NonNull<T>,
    cap: usize,
}

impl<T> Default for RawVector<T> {
    fn default() -> Self {
        Self {
            ptr: NonNull::dangling(),
            cap: 0,
        }
    }
}

impl<T> Drop for RawVector<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                dealloc(self.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T> RawVector<T> {
    fn new() -> Self {
        assert!(size_of::<T>() != 0, "ZST is not support");
        Self {
            ptr: NonNull::dangling(),
            cap: 0,
        }
    }

    fn with_cap(cap: usize) -> Self {
        assert!(size_of::<T>() != 0, "ZST is not support");

        let layout = if cap == 0 {
            Layout::array::<T>(1).unwrap()
        } else {
            Layout::array::<T>(cap).unwrap()
        };
        assert!(layout.size() <= isize::MAX as usize, "capacity too large");

        let ptr = match NonNull::new(unsafe { alloc(layout) } as *mut T) {
            Some(p) => p,
            None => handle_alloc_error(layout),
        };

        Self { ptr, cap }
    }

    fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = self.cap * 2;
            (new_cap, Layout::array::<T>(new_cap).unwrap())
        };

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "allocation too large"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.as_ptr() as *mut u8;
            unsafe { realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

#[derive(Debug)]
pub struct Vector<T> {
    buf: RawVector<T>,
    len: usize,
}

// trait impls

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Self {
            buf: Default::default(),
            len: 0,
        }
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T> {
    fn from(arr: [T; N]) -> Self {
        Self::from_slice(&arr)
    }
}

impl<T> From<&[T]> for Vector<T> {
    fn from(s: &[T]) -> Self {
        Self::from_slice(s)
    }
}

impl<T: PartialEq> PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        &self[..] == &other[..]
    }
}

impl<T: PartialEq, const N: usize> PartialEq<[T; N]> for Vector<T> {
    fn eq(&self, other: &[T; N]) -> bool {
        &self[..] == other
    }
}

impl<T: PartialEq> PartialEq<&[T]> for Vector<T> {
    fn eq(&self, other: &&[T]) -> bool {
        &self[..] == *other
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T: Display> Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, e) in self.iter().enumerate() {
            write!(f, "{}{}", e, if i == self.len - 1 { "" } else { ", " })?;
        }
        write!(f, "]")
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.len) }
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}

//

impl<T> Vector<T> {
    fn ptr(&self) -> *mut T {
        self.buf.as_ptr()
    }

    fn from_slice(s: &[T]) -> Self {
        let mut v = Vector::with_cap(s.len());
        unsafe {
            ptr::copy(s.as_ptr(), v.buf.as_ptr(), s.len());
        }
        v.len = s.len();
        v
    }
}

// public impls

impl<T> Vector<T> {
    pub fn new() -> Self {
        Self {
            buf: RawVector::new(),
            len: 0,
        }
    }

    pub fn with_cap(cap: usize) -> Self {
        Self {
            buf: RawVector::with_cap(cap),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn cap(&self) -> usize {
        self.buf.cap
    }

    pub fn clear(&mut self) {
        while let Some(_) = self.pop() {}
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        assert!(std::cmp::max(i, j) < self.len, "index out of bounds");
        if i != j {
            (self[i], self[j]) = unsafe {
                (
                    ptr::read(self.as_ptr().add(j)),
                    ptr::read(self.as_ptr().add(i)),
                )
            };
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.buf.grow();
        }

        unsafe {
            ptr::write(self.ptr().add(self.len), elem);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr().add(self.len))) }
        }
    }

    pub fn insert(&mut self, at: usize, elem: T) {
        assert!(at <= self.len, "index out of bounds");
        if self.len == self.cap() {
            self.buf.grow();
        }

        unsafe {
            ptr::copy(self.ptr().add(at), self.ptr().add(at + 1), self.len - at);
            ptr::write(self.ptr().add(at), elem);
        }

        self.len += 1;
    }

    pub fn remove(&mut self, at: usize) -> T {
        assert!(at < self.len, "index out of bounds");
        self.len -= 1;
        unsafe {
            let result = ptr::read(self.ptr().add(at));
            ptr::copy(self.ptr().add(at), self.ptr().add(at + 1), self.len - at);
            result
        }
    }
}

// iterator impls

#[derive(Debug)]
pub struct IntoIter<T> {
    _buf: RawVector<T>,
    start: *const T,
    end: *const T,
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(result)
            }
        }
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = ptr::read(self.end);
                self.end = self.end.offset(-1);
                Some(result)
            }
        }
    }
}

impl<T> IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let buf = unsafe { ptr::read(&self.buf) };
        let len = self.len();
        std::mem::forget(self);

        IntoIter {
            start: buf.as_ptr(),
            end: if buf.cap == 0 {
                buf.as_ptr()
            } else {
                unsafe { buf.as_ptr().add(len) }
            },
            _buf: buf,
        }
    }
}
