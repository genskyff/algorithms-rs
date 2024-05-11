//! Len of **main string** is `n` and **pattern string** is `m`
//! - **Best** time complexity ---- O(n + m)
//! - **Worst** time complexity ---- O(n + m)
//! - **Average** time complexity ---- O(n + m)

pub trait Kmp {
    type Pattern;
    fn kmp(&self, pattern: Self::Pattern) -> Vec<usize>;
}

impl<T: AsRef<str>> Kmp for T {
    type Pattern = T;

    fn kmp(&self, pattern: Self::Pattern) -> Vec<usize> {
        todo!()
    }
}
