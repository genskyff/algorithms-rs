//! - **Category** ---- Comparison-based
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

impl<T: Ord + Clone + Debug> Insertion for [T] {
    fn insertion_sort(&mut self) {
        let len = self.len();

        for i in 1..len {
            let base = self[i].clone();
            let mut j = i;

            while j > 0 && self[j - 1] > base {
                self[j] = self[j - 1].clone();
                j -= 1;
            }

            self[j] = base;
        }
    }

    fn binary_insertion_sort(&mut self) {
        let len = self.len();

        for i in 1..len {
            let base = self[i].clone();
            let mut low = 0;
            let mut high = i;

            while low < high {
                let mid = (low + high) / 2;

                if self[mid] > base {
                    high = mid;
                } else {
                    low = mid + 1;
                }
            }

            self[low..i + 1].rotate_right(1);
            self[low] = base;
        }
    }

    fn shell_sort(&mut self) {
        let len = self.len();
        let mut gap = 1;

        // Simplified Knuth's sequence
        while gap < len / 3 {
            gap = gap * 3 + 1;
        }

        while gap >= 1 {
            for i in gap..len {
                let base = self[i].clone();
                let mut j = i;

                while gap <= j && self[j - gap] > base {
                    self[j] = self[j - gap].clone();
                    j -= gap;
                }

                self[j] = base;
            }

            gap = (gap - 1) / 3;
        }
    }
}
