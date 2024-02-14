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

fn qsort<T: Ord + Copy + Debug>(arr: &mut [T], mut lo: usize, mut hi: usize) {
    while lo < hi {
        let pivot = partition(arr, lo, hi);

        if pivot - lo < hi - pivot {
            if pivot > 0 {
                qsort(arr, lo, pivot - 1);
            }
            lo = pivot + 1;
        } else {
            qsort(arr, pivot + 1, hi);
            hi = pivot - 1;
        }

        #[cfg(feature = "debug-print")]
        println!("next:\t{arr:?}");
    }
}

fn partition<T: Ord + Copy + Debug>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    move_pivot_to_right(arr, lo, hi);
    let pivot = arr[hi];
    let mut tail = lo;

    for i in lo..hi {
        if arr[i] <= pivot {
            arr.swap(i, tail);
            tail += 1;
        }
    }
    arr.swap(tail, hi);

    tail
}

fn move_pivot_to_right<T: Ord + Copy + Debug>(arr: &mut [T], lo: usize, hi: usize) {
    let mid = lo + (hi - lo) / 2;

    // move the median of lo, mid, hi to the hi (except hi is the median)
    if (arr[lo] < arr[mid]) ^ (arr[lo] < arr[hi]) {
        arr.swap(lo, hi);
    } else if (arr[mid] < arr[lo]) ^ (arr[mid] < arr[hi]) {
        arr.swap(mid, hi);
    }
}
