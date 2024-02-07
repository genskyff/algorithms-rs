//! - **Category** ---- Comparison-based
//! - **Data structure** ---- Array
//! - **Stability** ---- Yes
//! - **In-place** ---- Yes
//! - **Space** complexity ---- O(1)
//! - **Adaptiveness** ---- Yes
//! - **Worst** time complexity ---- O(n<sup>2</sup>)
//! - **Average** time complexity ---- O(n<sup>2</sup>)

use std::fmt::Debug;

pub trait Insertion {
    /// - **Best** time complexity ---- O(n)
    fn insertion_sort(&mut self);

    /// - **Best** time complexity ---- O(nlogn)
    fn binary_insertion_sort(&mut self);

    /// - **Stability** ---- No
    ///
    /// Time complexity depends on the **gap sequence**
    /// - **Best** time complexity ---- O(nlogn) ~ O(nlog<sup>2</sup>n)
    /// - **Worst** time complexity ---- O(nlog<sup>2</sup>n) ~ O(n<sup>2</sup>)
    /// - **Average** time complexity ---- O(nlog<sup>2</sup>n) ~ O(n<sup>3/2</sup>)
    fn shell_sort(&mut self);
}

impl<T: Ord + Copy + Debug> Insertion for [T] {
    fn insertion_sort(&mut self) {
        let len = self.len();

        #[cfg(feature = "debug-print")]
        println!("\nbefore:\t{self:?}");

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
            println!("next: \t{self:?}");
        }
    }

    fn binary_insertion_sort(&mut self) {
        let len = self.len();

        #[cfg(feature = "debug-print")]
        println!("\nbefore:\t{self:?}");

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
            println!("next:\t{self:?}");
        }
    }

    fn shell_sort(&mut self) {
        let len = self.len();

        #[cfg(feature = "debug-print")]
        println!("\nbefore:\t{self:?}");

        if len < 2 {
            return;
        }

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
            println!("next:\t{self:?}");
        }
    }
}
