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

impl<T: Ord + Copy + Debug> Bubble for [T] {
    fn bubble_sort(&mut self) {
        let len = self.len();

        #[cfg(feature = "debug-print")]
        println!("\nbefore:\t{self:?}");

        if len < 2 {
            return;
        }

        for i in 0..len {
            let mut flag = false;

            for j in 0..len - i - 1 {
                if self[j] > self[j + 1] {
                    self.swap(j, j + 1);
                    flag = true;
                }
            }

            #[cfg(feature = "debug-print")]
            println!("next:\t{self:?}");

            if !flag {
                break;
            }
        }
    }

    fn cocktail_sort(&mut self) {
        let len = self.len();

        #[cfg(feature = "debug-print")]
        println!("\nbefore:\t{self:?}");

        if len < 2 {
            return;
        }

        let mut start = 0;
        let mut end = len - 1;

        while start < end {
            let mut flag = false;

            for i in start..end {
                if self[i] > self[i + 1] {
                    self.swap(i, i + 1);
                    flag = true;
                }
            }

            end -= 1;

            for i in (start + 1..=end).rev() {
                if self[i] < self[i - 1] {
                    self.swap(i, i - 1);
                    flag = true;
                }
            }

            #[cfg(feature = "debug-print")]
            println!("next:\t{self:?}");

            if !flag {
                break;
            }

            start += 1;
        }
    }
}
