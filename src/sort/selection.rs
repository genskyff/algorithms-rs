//! - **Category** ---- Comparison-based
//! - **Data structure** ---- Array
//! - **Stability** ---- No
//! - **In-place** ---- Yes
//! - **Space** complexity ---- O(1)
//! - **Adaptiveness** ---- No
//! - **Time** complexity ---- O(n<sup>2</sup>)

use std::fmt::Debug;

pub trait Selection {
    fn selection_sort(&mut self);
}

impl<T: Ord + Debug> Selection for [T] {
    fn selection_sort(&mut self) {
        if self.is_empty() {
            return;
        }

        let len = self.len();
        for i in 0..(len - 1) {
            let mut min = i;

            for j in i + 1..len {
                if self[j] < self[min] {
                    min = j;
                }
            }

            self.swap(i, min);
        }
    }
}
