//! - Category ---- Comparison-based sorting
//! - Data structure ---- Array
//! - Worst-case time complexity ---- O(n<sup>2</sup>)
//! - Average time complexity ---- O(n<sup>2</sup>)
//! - Space complexity ---- O(1)
//! - Stability ---- Stable

pub trait Bubble {
    /// Optimization —— Use a flag to indicate whether a swap is needed
    fn bubble_sort(&mut self);

    /// Optimization —— Perform bubble sort in both directions
    fn cocktail_sort(&mut self);
}

impl<T: PartialOrd> Bubble for [T] {
    fn bubble_sort(&mut self) {
        let len = self.len();

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

            if !flag {
                break;
            }
        }
    }

    fn cocktail_sort(&mut self) {
        let len = self.len();

        if len < 2 {
            return;
        }

        let mut start = 0usize;
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

            if !flag {
                break;
            }

            start += 1;
        }
    }
}
