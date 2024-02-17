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

impl<T: Ord + Clone + Debug> Merge for [T] {
    fn merge_sort_recu(&mut self) {
        if self.is_empty() {
            return;
        }

        let len = self.len();
        let mut tmp = vec![self[0].clone(); len];
        msort_recu(self, &mut tmp[..], 0, len - 1);
    }

    fn merge_sort_iter(&mut self) {
        if self.is_empty() {
            return;
        }

        let len = self.len();
        let mut tmp = vec![self[0].clone(); len];
        msort_iter(self, &mut tmp[..], len);
    }
}

fn msort_recu<T: Ord + Clone + Debug>(arr: &mut [T], tmp: &mut [T], low: usize, high: usize) {
    if low < high {
        let mid = low + (high - low) / 2;

        msort_recu(arr, tmp, low, mid);
        msort_recu(arr, tmp, mid + 1, high);
        merge(arr, tmp, low, mid, high);
    }
}

fn msort_iter<T: Ord + Clone + Debug>(arr: &mut [T], tmp: &mut [T], len: usize) {
    let (mut low, mut mid, mut high);

    let mut i = 1;
    while i < len {
        low = 0;
        while low + i < len {
            mid = low + i - 1;
            high = if mid + i < len { mid + i } else { len - 1 };
            merge(arr, tmp, low, mid, high);
            low = high + 1;
        }
        i *= 2;
    }
}

fn merge<T: Ord + Clone + Debug>(
    arr: &mut [T],
    tmp: &mut [T],
    low: usize,
    mid: usize,
    high: usize,
) {
    let (mut l_pos, mut h_pos, mut t_pos) = (low, mid + 1, low);

    while l_pos <= mid && h_pos <= high {
        if arr[l_pos] < arr[h_pos] {
            tmp[t_pos] = arr[l_pos].clone();
            l_pos += 1;
        } else {
            tmp[t_pos] = arr[h_pos].clone();
            h_pos += 1;
        }
        t_pos += 1;
    }

    while l_pos <= mid {
        tmp[t_pos] = arr[l_pos].clone();
        l_pos += 1;
        t_pos += 1;
    }

    while h_pos <= high {
        tmp[t_pos] = arr[h_pos].clone();
        h_pos += 1;
        t_pos += 1;
    }

    for i in low..t_pos {
        arr[i] = tmp[i].clone();
    }
}
