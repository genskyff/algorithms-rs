//! - **Category** ---- Comparison-based
//! - **Data structure** ---- Array
//! - **Stability** ---- No
//! - **In-place** ---- Yes
//! - **Space** complexity ---- O(logn) ~ O(n)
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

fn qsort<T: Ord + Copy + Debug>(arr: &mut [T], mut left: usize, mut right: usize) {
    while left < right {
        let pivot = partition(arr, left, right);

        if pivot - left < right - pivot {
            if pivot >= 1 {
                qsort(arr, left, pivot - 1);
            }
            left = pivot + 1;
        } else {
            qsort(arr, pivot + 1, right);
            right = pivot - 1;
        }

        #[cfg(feature = "debug-print")]
        println!("next:\t{arr:?}");
    }
}

fn partition<T: Ord + Copy + Debug>(arr: &mut [T], left: usize, right: usize) -> usize {
    move_pivot_to_right(arr, left, right);
    let pivot = arr[right];
    let mut tail = left;

    for i in left..right {
        if arr[i] <= pivot {
            arr.swap(i, tail);
            tail += 1;
        }
    }
    arr.swap(tail, right);

    tail
}

fn move_pivot_to_right<T: Ord + Copy + Debug>(arr: &mut [T], left: usize, right: usize) {
    let mid = left + (right - left) / 2;

    // move the median of left, mid, right to the right (except right is the median)
    if (arr[left] < arr[mid]) ^ (arr[left] < arr[right]) {
        arr.swap(left, right);
    } else if (arr[mid] < arr[left]) ^ (arr[mid] < arr[right]) {
        arr.swap(mid, right);
    }
}
