//! - **Category** ---- Comparison-based
//! - **Data structure** ---- Array
//! - **Stability** ---- Yes
//! - **In-place** ---- No
//! - **Space** complexity ---- O(n)
//! - **Adaptiveness** ---- No
//! - **Time** complexity ---- O(nlogn)

use std::fmt::Debug;

pub trait Merge {
    fn merge_sort_recu(&mut self);

    fn merge_sort_iter(&mut self);
}

impl<T: Ord + Copy + Debug> Merge for [T] {
    fn merge_sort_recu(&mut self) {
        let len = self.len();

        #[cfg(feature = "debug-print")]
        println!("\nbegin:\t{self:?}");

        if len < 2 {
            return;
        }

        let mut tmp = vec![self[0]; len];
        msort_recu(self, &mut tmp[..], 0, len - 1);
    }

    fn merge_sort_iter(&mut self) {
        let len = self.len();

        #[cfg(feature = "debug-print")]
        println!("\nbegin:\t{self:?}");

        if len < 2 {
            return;
        }

        let mut tmp = vec![self[0]; len];
        msort_iter(self, &mut tmp[..], len);
    }
}

fn msort_recu<T: Ord + Copy + Debug>(arr: &mut [T], tmp: &mut [T], left: usize, right: usize) {
    if left < right {
        let mid = left + (right - left) / 2;

        msort_recu(arr, tmp, left, mid);
        msort_recu(arr, tmp, mid + 1, right);
        merge(arr, tmp, left, mid, right);
    }
}

fn msort_iter<T: Ord + Copy + Debug>(arr: &mut [T], tmp: &mut [T], len: usize) {
    let (mut left, mut mid, mut right);

    let mut i = 1;
    while i < len {
        left = 0;
        while left + i < len {
            mid = left + i - 1;
            right = if mid + i < len { mid + i } else { len - 1 };
            merge(arr, tmp, left, mid, right);
            left = right + 1;
        }
        i *= 2;
    }
}

fn merge<T: Ord + Copy + Debug>(
    arr: &mut [T],
    tmp: &mut [T],
    left: usize,
    mid: usize,
    right: usize,
) {
    let (mut l_pos, mut r_pos, mut t_pos) = (left, mid + 1, left);

    while l_pos <= mid && r_pos <= right {
        if arr[l_pos] < arr[r_pos] {
            tmp[t_pos] = arr[l_pos];
            l_pos += 1;
        } else {
            tmp[t_pos] = arr[r_pos];
            r_pos += 1;
        }
        t_pos += 1;
    }

    while l_pos <= mid {
        tmp[t_pos] = arr[l_pos];
        l_pos += 1;
        t_pos += 1;
    }

    while r_pos <= right {
        tmp[t_pos] = arr[r_pos];
        r_pos += 1;
        t_pos += 1;
    }

    for i in left..t_pos {
        arr[i] = tmp[i];
    }

    #[cfg(feature = "debug-print")]
    println!("next:\t{arr:?}");
}
