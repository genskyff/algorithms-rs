//! - Category ---- Comparison-based sorting
//! - Data structure ---- Array
//! - Worst-case time complexity ---- O(n<sup>2</sup>)
//! - Average time complexity ---- O(n<sup>2</sup>)
//! - Space complexity ---- O(1)
//! - Stability ---- Stable

use std::fmt::Debug;

pub trait Insertion {
    fn insertion_sort(&mut self);

    fn binary_insertion_sort(&mut self);

    fn shell_sort(&mut self);
}

impl<T: Ord + Copy + Debug> Insertion for [T] {
    fn insertion_sort(&mut self) {
        let len = self.len();

        #[cfg(feature = "debug-print")]
        println!("\nbefore: {self:?}");

        if len < 2 {
            return;
        }

        for i in 1..len {
            let base = self[i];
            let mut j = i;

            while j > 0 && self[j - 1] > base {
                self[j] = self[j - 1];
                j -= 1;
            }

            self[j] = base;

            #[cfg(feature = "debug-print")]
            println!("next: {self:?}");
        }
    }

    fn binary_insertion_sort(&mut self) {
        todo!()
    }

    fn shell_sort(&mut self) {
        todo!()
    }
}
