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

impl<T: Ord + Copy + Debug> Selection for [T] {
    fn selection_sort(&mut self) {
        let len = self.len();

        #[cfg(feature = "debug-print")]
        println!("\nbefore:\t{self:?}");

        if len < 2 {
            return;
        }

        for i in 0..len - 1 {
            let mut min = i;

            for j in i + 1..len {
                if self[j] < self[min] {
                    min = j;
                }
            }

            if i != min {
                self.swap(i, min);
            }

            #[cfg(feature = "debug-print")]
            println!("next:\t{self:?}");
        }
    }
}
