//! - **Category** ---- Non-comparison-based
//! - **Data structure** ---- Array
//! - **Stability** ---- Yes
//! - **In-place** ---- No
//! - **Space** complexity ---- O(n+k)
//! - **Adaptiveness** ---- No
//! - **Time** complexity ---- O(n+k)

use std::fmt::Debug;

pub trait Counting {
    fn counting_sort(&mut self);
}

impl<T: Ord + Copy + Debug> Counting for [T] {
    fn counting_sort(&mut self) {
        todo!()
    }
}
