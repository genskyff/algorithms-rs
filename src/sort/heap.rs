//! - **Category** ---- Comparison-based
//! - **Data structure** ---- Array
//! - **Stability** ---- No
//! - **In-place** ---- Yes
//! - **Space** complexity ---- O(1)
//! - **Adaptiveness** ---- No
//! - **Time** complexity ---- O(nlogn)

use std::fmt::Debug;

pub trait Heap {
    fn heap_sort(&mut self);
}

impl<T: Ord + Copy + Debug> Heap for [T] {
    fn heap_sort(&mut self) {
        todo!()
    }
}
