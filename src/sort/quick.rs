//! - **Category** ---- Comparison-based
//! - **Data structure** ---- Array
//! - **Stability** ---- No
//! - **In-place** ---- Yes
//! - **Space** complexity ---- O(n)
//! - **Adaptiveness** ---- Yes
//! - **Best** time complexity ---- O(nlogn)
//! - **Worst** time complexity ---- O(n<sup>2</sup>)
//! - **Average** time complexity ---- O(nlogn)

use std::fmt::Debug;

pub trait Quick {
    fn quick_sort(&mut self);
}

impl<T: Ord + Copy + Debug> Quick for [T] {
    fn quick_sort(&mut self) {
        todo!()
    }
}
