//! - **Category** ---- Comparison-based
//! - **Data structure** ---- Array
//! - **Stability** ---- No
//! - **In-place** ---- Yes
//! - **Space** complexity ---- O(n)
//! - **Adaptiveness** ---- Yes
//! - **Best** time complexity ---- O(nlogn)
//! - **Worst** time complexity ---- O(n<sup>2</sup>)
//! - **Average** time complexity ---- O(nlogn)

use std::fmt::Debug;

pub trait Quick {
    fn quick_sort(&mut self);
}

impl<T: Ord + Copy + Debug> Quick for [T] {
    fn quick_sort(&mut self) {
        let len = self.len();

        #[cfg(feature = "debug-print")]
        println!("\nbegin:\t{self:?}");

        if len < 2 {
            return;
        }

        qsort(self, 0, len - 1);
    }
}

fn qsort<T: Ord + Copy + Debug>(arr: &mut [T], left: usize, right: usize) {
    if left >= right {
        return;
    }

    let pivot = partition(arr, left, right);
    if pivot > 0 {
        qsort(arr, left, pivot - 1);
    }
    qsort(arr, pivot + 1, right);
}

fn partition<T: Ord + Copy + Debug>(arr: &mut [T], left: usize, right: usize) -> usize {
    let pivot = arr[right];
    let mut tail = left;
    for i in left..right {
        if arr[i] <= pivot {
            arr.swap(tail, i);
            tail += 1;
        }
    }
    arr.swap(tail, right);

    tail
}
