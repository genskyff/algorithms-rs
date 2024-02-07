//! - **Category** ---- Comparison-based
//! - **Data structure** ---- Array
//! - **Stability** ---- Yes
//! - **In-place** ---- No
//! - **Space** complexity ---- O(n)
//! - **Adaptiveness** ---- No
//! - **Time** complexity ---- O(nlogn)

use std::fmt::Debug;

pub trait Merge {
    fn merge_sort(&mut self);
}

impl<T: Ord + Copy + Debug> Merge for [T] {
    fn merge_sort(&mut self) {
        todo!()
    }
}
