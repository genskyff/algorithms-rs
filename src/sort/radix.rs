//! - **Category** ---- Non-comparison-based
//! - **Stability** ---- Yes
//! - **In-place** ---- No
//! - **Space** complexity ---- O(n+k)
//! - **Adaptiveness** ---- No
//! - **Time** complexity ---- O(nk)

use std::fmt::Debug;

pub trait Radix {
    fn radix_sort(&mut self);
}

impl<T: Ord + Copy + Debug> Radix for [T] {
    fn radix_sort(&mut self) {
        todo!()
    }
}
