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
        println!("\nbegin:\t{self:?}");

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
        println!("\nbegin:\t{self:?}");

        if len < 2 {
            return;
        }

        for i in 1..len {
            let base = self[i];
            let mut lo = 0;
            let mut hi = i;

            while lo < hi {
                let mid = (lo + hi) / 2;

                if self[mid] > base {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }

            for j in (lo..i).rev() {
                self[j + 1] = self[j];
            }

            self[lo] = base;

            #[cfg(feature = "debug-print")]
            println!("next:\t{self:?}");
        }
    }

    fn shell_sort(&mut self) {
        let len = self.len();

        #[cfg(feature = "debug-print")]
        println!("\nbegin:\t{self:?}");

        if len < 2 {
            return;
        }

        let mut gap = 1;

        // Simplified Knuth's sequence
        while gap < len / 3 {
            gap = gap * 3 + 1;
        }

        while gap >= 1 {
            for i in gap..len {
                let base = self[i];
                let mut j = i;

                while gap <= j && self[j - gap] > base {
                    self[j] = self[j - gap];
                    j -= gap;
                }

                self[j] = base;
            }

            gap = (gap - 1) / 3;

            #[cfg(feature = "debug-print")]
            println!("next:\t{self:?}");
        }
    }
}
