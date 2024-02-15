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

        if self.is_empty() {
            return;
        }

        let mut tmp = vec![self[0]; len];
        msort_recu(self, &mut tmp[..], 0, len - 1);
    }

    fn merge_sort_iter(&mut self) {
        let len = self.len();

        #[cfg(feature = "debug-print")]
        println!("\nbegin:\t{self:?}");

        if self.is_empty() {
            return;
        }

        let mut tmp = vec![self[0]; len];
        msort_iter(self, &mut tmp[..], len);
    }
}

fn msort_recu<T: Ord + Copy + Debug>(arr: &mut [T], tmp: &mut [T], lo: usize, hi: usize) {
    if lo < hi {
        let mid = lo + (hi - lo) / 2;

        msort_recu(arr, tmp, lo, mid);
        msort_recu(arr, tmp, mid + 1, hi);
        merge(arr, tmp, lo, mid, hi);
    }
}

fn msort_iter<T: Ord + Copy + Debug>(arr: &mut [T], tmp: &mut [T], len: usize) {
    let (mut lo, mut mid, mut hi);

    let mut i = 1;
    while i < len {
        lo = 0;
        while lo + i < len {
            mid = lo + i - 1;
            hi = if mid + i < len { mid + i } else { len - 1 };
            merge(arr, tmp, lo, mid, hi);
            lo = hi + 1;
        }
        i *= 2;
    }
}

fn merge<T: Ord + Copy + Debug>(
    arr: &mut [T],
    tmp: &mut [T],
    lo: usize,
    mid: usize,
    hi: usize,
) {
    let (mut l_pos, mut h_pos, mut t_pos) = (lo, mid + 1, lo);

    while l_pos <= mid && h_pos <= hi {
        if arr[l_pos] < arr[h_pos] {
            tmp[t_pos] = arr[l_pos];
            l_pos += 1;
        } else {
            tmp[t_pos] = arr[h_pos];
            h_pos += 1;
        }
        t_pos += 1;
    }

    while l_pos <= mid {
        tmp[t_pos] = arr[l_pos];
        l_pos += 1;
        t_pos += 1;
    }

    while h_pos <= hi {
        tmp[t_pos] = arr[h_pos];
        h_pos += 1;
        t_pos += 1;
    }

    for i in lo..t_pos {
        arr[i] = tmp[i];
    }

    #[cfg(feature = "debug-print")]
    println!("next:\t{arr:?}");
}
