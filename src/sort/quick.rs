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

impl<T: Ord + Debug> Quick for [T] {
    fn quick_sort(&mut self) {
        let len = self.len();

        #[cfg(feature = "debug-print")]
        println!("\nbegin:\t{self:?}");

        if self.is_empty() {
            return;
        }

        qsort(self, 0, len - 1);
    }
}

fn qsort<T: Ord + Debug>(arr: &mut [T], mut low: usize, mut high: usize) {
    while low < high {
        let pivot = partition(arr, low, high);

        if pivot - low < high - pivot {
            if pivot > 0 {
                qsort(arr, low, pivot - 1);
            }
            low = pivot + 1;
        } else {
            qsort(arr, pivot + 1, high);
            high = pivot - 1;
        }

        #[cfg(feature = "debug-print")]
        println!("next:\t{arr:?}");
    }
}

fn partition<T: Ord + Debug>(arr: &mut [T], low: usize, high: usize) -> usize {
    move_pivot_to_right(arr, low, high);
    let mut cur = low;

    for i in low..high {
        if arr[i] <= arr[high] {
            arr.swap(i, cur);
            cur += 1;
        }
    }
    arr.swap(cur, high);

    cur
}

fn move_pivot_to_right<T: Ord + Debug>(arr: &mut [T], low: usize, high: usize) {
    let mid = low + (high - low) / 2;

    // move the median of low, mid, high to the high
    let pivot = if (arr[low] < arr[mid]) ^ (arr[low] < arr[high]) {
        low
    } else if (arr[mid] < arr[low]) ^ (arr[mid] < arr[high]) {
        mid
    } else {
        high
    };

    arr.swap(pivot, high);
}
