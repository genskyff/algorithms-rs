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
        println!("\nstart: {self:?}");

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
            println!("next:  {self:?}");
        }
    }

    fn binary_insertion_sort(&mut self) {
        let len = self.len();

        #[cfg(feature = "debug-print")]
        println!("\nstart: {self:?}");

        if len < 2 {
            return;
        }

        for i in 1..len {
            let base = self[i];
            let mut left = 0;
            let mut right = i;

            while left < right {
                let mid = (left + right) / 2;

                if self[mid] > base {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }

            for j in (left..i).rev() {
                self[j + 1] = self[j];
            }

            self[left] = base;

            #[cfg(feature = "debug-print")]
            println!("next:  {self:?}");
        }
    }

    fn shell_sort(&mut self) {
        let len = self.len();

        #[cfg(feature = "debug-print")]
        println!("\nstart: {self:?}");

        let mut h = 1;

        while h < len / 3 {
            h = 3 * h + 1;
        }

        while h >= 1 {
            for i in h..len {
                let base = self[i];
                let mut j = i;

                while j >= h && self[j - h] > base {
                    self[j] = self[j - h];
                    j -= h;
                }
                self[j] = base;
            }

            h /= 3;

            #[cfg(feature = "debug-print")]
            println!("next:  {self:?}");
        }
    }
}
