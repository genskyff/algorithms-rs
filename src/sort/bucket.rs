//! - **Category** ---- Non-comparison-based
//! - **Data structure** ---- Array
//! - **Stability** ---- Depends on the internal sorting algorithm
//! - **In-place** ---- No
//! - **Space** complexity ---- O(n+k)
//! - **Adaptiveness** ---- Depends on the internal sorting algorithm
//! - **Best** time complexity ---- O(n+k)
//! - **Worst** time complexity ---- O(n<sup>2</sup>)
//! - **Average** time complexity ---- O(n+k)

use std::fmt::Debug;

pub trait Bucket {
    fn bucket_sort(&mut self);
}

impl<T: Ord + Copy + Debug> Bucket for [T] {
    fn bucket_sort(&mut self) {
        todo!()
    }
}
