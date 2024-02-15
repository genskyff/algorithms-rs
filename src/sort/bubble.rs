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
        println!("\nbegin:\t{self:?}");

        if len < 2 {
            return;
        }

        for i in 0..len {
            let mut swapped = false;

            for j in 0..(len - i - 1) {
                if self[j] > self[j + 1] {
                    self.swap(j, j + 1);
                    swapped = true;
                }
            }

            #[cfg(feature = "debug-print")]
            println!("next:\t{self:?}");

            if !swapped {
                break;
            }
        }
    }

    fn cocktail_sort(&mut self) {
        let len = self.len();

        #[cfg(feature = "debug-print")]
        println!("\nbegin:\t{self:?}");

        if self.is_empty() {
            return;
        }

        let mut low = 0;
        let mut high = len - 1;

        while low < high {
            let mut swapped = false;

            for i in low..high {
                if self[i] > self[i + 1] {
                    self.swap(i, i + 1);
                    swapped = true;
                }
            }

            if !swapped {
                #[cfg(feature = "debug-print")]
                println!("next:\t{self:?}");
                break;
            }

            high -= 1;
            swapped = false;

            for i in ((low + 1)..=high).rev() {
                if self[i] < self[i - 1] {
                    self.swap(i, i - 1);
                    swapped = true;
                }
            }

            #[cfg(feature = "debug-print")]
            println!("next:\t{self:?}");

            if !swapped {
                break;
            }

            low += 1;
        }
    }
}
