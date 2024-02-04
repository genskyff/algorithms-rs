//! - Category ---- Comparison-based sorting
//! - Data structure ---- Array
//! - Worst-case time complexity ---- O(n<sup>2</sup>)
//! - Average time complexity ---- O(n<sup>2</sup>)
//! - Space complexity ---- O(1)
//! - Stability ---- Unstable

pub trait Selection {
    fn selection_sort(&mut self);
}

impl<T: PartialOrd> Selection for [T] {
    fn selection_sort(&mut self) {
        let len = self.len();

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
        }
    }
}
