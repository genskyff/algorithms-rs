//! Length of **main string** is `n` and **pattern string** is `m`
//! - **Best** time complexity ---- O(n)
//! - **Worst** time complexity ---- O(nm)
//! - **Average** time complexity ---- O(n)

pub trait BruteForce {
    type Pattern;
    fn brute_force(&self, pattern: Self::Pattern) -> Vec<usize>;
}

impl<T: AsRef<str>> BruteForce for T {
    type Pattern = T;

    fn brute_force(&self, pattern: Self::Pattern) -> Vec<usize> {
        let s = self.as_ref();
        let pat = pattern.as_ref();
        let n = s.len();
        let m = pat.len();
        let mut indices = Vec::new();

        if n < m {
            return indices;
        }

        for i in 0..n - m + 1 {
            let mut j = 0;
            while j < m && s.chars().nth(i + j).unwrap() == pat.chars().nth(j).unwrap() {
                j += 1;
            }
            if j == m {
                indices.push(i);
            }
        }

        indices
    }
}
