//! - 分类 —— 基于比较的排序
//! - 数据结构 —— 数组
//! - 最差时间复杂度 —— O(n<sup>2</sup>)
//! - 平均时间复杂度 —— O(n<sup>2</sup>)
//! - 最优时间复杂度 —— O(n)
//! - 就地性 —— O(1)
//! - 稳定性 —— 稳定

pub trait Bubble {
    /// 效率优化 —— 使用 flag 来表示是否需要交换
    fn bubble_sort(&mut self);

    /// 效率优化 —— 双向进行排序操作
    fn cocktail_sort(&mut self);
}

impl<T: PartialOrd> Bubble for [T] {
    fn bubble_sort(&mut self) {
        let len = self.len();

        for i in 0..len {
            let mut flag = false;

            for j in 0..len - i - 1 {
                if self[j] > self[j + 1] {
                    self.swap(j, j + 1);
                    flag = true;
                }
            }

            if !flag {
                break;
            }
        }
    }

    fn cocktail_sort(&mut self) {
        let mut start = 0usize;
        let mut end = self.len() - 1;

        while start < end {
            let mut flag = false;

            for i in start..end {
                if self[i] > self[i + 1] {
                    self.swap(i, i + 1);
                    flag = true;
                }
            }

            end -= 1;

            for i in (start + 1..=end).rev() {
                if self[i] < self[i - 1] {
                    self.swap(i, i - 1);
                    flag = true;
                }
            }

            if !flag {
                break;
            }

            start += 1;
        }
    }
}
