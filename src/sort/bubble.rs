//! - **Category** ---- Comparison-based
//! - **Data structure** ---- Array
//! - **Stability** ---- Yes
//! - **In-place** ---- Yes
//! - **Space** complexity ---- O(1)
//! - **Adaptiveness** ---- Yes
//! - **Best** time complexity ---- O(n)
//! - **Worst** time complexity ---- O(n<sup>2</sup>)
//! - **Average** time complexity ---- O(n<sup>2</sup>)

use std::fmt::Debug;

pub trait Bubble {
    fn bubble_sort(&mut self);

    fn cocktail_sort(&mut self);
}

impl<T: Ord + Debug> Bubble for [T] {
    fn bubble_sort(&mut self) {
        let len = self.len();

        for i in 0..len {
            let mut is_swapped = false;

            for j in 0..len - i - 1 {
                if self[j] > self[j + 1] {
                    self.swap(j, j + 1);
                    is_swapped = true;
                }
            }

            if !is_swapped {
                break;
            }
        }
    }

    fn cocktail_sort(&mut self) {
        if self.is_empty() {
            return;
        }

        let len = self.len();
        let mut low = 0;
        let mut high = len - 1;

        while low < high {
            let mut is_swapped = false;

            for i in low..high {
                if self[i] > self[i + 1] {
                    self.swap(i, i + 1);
                    is_swapped = true;
                }
            }

            if !is_swapped {
                break;
            }

            high -= 1;
            is_swapped = false;

            for i in (low + 1..=high).rev() {
                if self[i] < self[i - 1] {
                    self.swap(i, i - 1);
                    is_swapped = true;
                }
            }

            if !is_swapped {
                break;
            }

            low += 1;
        }
    }
}
