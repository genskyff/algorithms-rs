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
        println!("\nbefore:\t{self:?}");

        if len < 2 {
            return;
        }

        entry_recu(self, len);
    }

    fn merge_sort_iter(&mut self) {
        let len = self.len();

        #[cfg(feature = "debug-print")]
        println!("\nbefore:\t{self:?}");

        if len < 2 {
            return;
        }

        entry_iter(self, len);
    }
}

fn entry_recu<T: Ord + Copy + Debug>(items: &mut [T], len: usize) {
    let mut tmp = vec![items[0]; len];
    divide_recu(items, 0, len - 1, &mut tmp[..]);
}

fn entry_iter<T: Ord + Copy + Debug>(items: &mut [T], len: usize) {
    let mut tmp = vec![items[0]; len];
    divide_iter(items, len, &mut tmp[..]);
}

fn divide_recu<T: Ord + Copy + Debug>(items: &mut [T], left: usize, right: usize, tmp: &mut [T]) {
    if left < right {
        let mid = (left + right) / 2;

        divide_recu(items, left, mid, tmp);
        divide_recu(items, mid + 1, right, tmp);
        conquer(items, left, mid, right, tmp);
    }
}

fn divide_iter<T: Ord + Copy + Debug>(items: &mut [T], len: usize, tmp: &mut [T]) {
    let (mut left, mut mid, mut right);

    let mut i = 1;
    while i < len {
        left = 0;
        while left + i < len {
            mid = left + i - 1;
            right = if mid + i < len { mid + i } else { len - 1 };
            conquer(items, left, mid, right, tmp);
            left = right + 1;
        }
        i *= 2;
    }
}

fn conquer<T: Ord + Copy + Debug>(
    items: &mut [T],
    left: usize,
    mid: usize,
    right: usize,
    tmp: &mut [T],
) {
    let (mut i, mut j, mut k) = (left, mid + 1, 0);

    while i <= mid && j <= right {
        if items[i] < items[j] {
            tmp[k] = items[i];
            i += 1;
        } else {
            tmp[k] = items[j];
            j += 1;
        }
        k += 1;
    }

    while i <= mid {
        tmp[k] = items[i];
        i += 1;
        k += 1;
    }

    while j <= right {
        tmp[k] = items[j];
        j += 1;
        k += 1;
    }

    for i in 0..k {
        items[left + i] = tmp[i];
    }

    #[cfg(feature = "debug-print")]
    println!("next:\t{items:?}");
}
