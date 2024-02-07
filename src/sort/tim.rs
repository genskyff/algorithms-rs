//! - **Category** ---- Comparison-based
//! - **Data structure** ---- Array
//! - **Stability** ---- Yes
//! - **In-place** ---- No
//! - **Space** complexity ---- O(n)
//! - **Adaptiveness** ---- Yes
//! - **Best** time complexity ---- O(n)
//! - **Worst** time complexity ---- O(nlogn)
//! - **Average** time complexity ---- O(nlogn)

use std::fmt::Debug;

pub trait Tim {
    fn tim_sort(&mut self);
}

impl<T: Ord + Copy + Debug> Tim for [T] {
    fn tim_sort(&mut self) {
        todo!()
    }
}
