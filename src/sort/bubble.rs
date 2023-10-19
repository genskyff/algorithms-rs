//! - 分类 —— 基于比较的排序
//! - 数据结构 —— 数组
//! - 最差时间复杂度 —— O(n<sup>2</sup>)
//! - 平均时间复杂度 —— O(n<sup>2</sup>)
//! - 就地性 —— O(1)
//! - 稳定性 —— 稳定

pub trait Bubble {
    /// 最优时间复杂度 —— O(n<sup>2</sup>)
    fn bubble(&mut self) {}

    /// 效率优化 —— 使用 flag 来表示是否需要交换 <br>
    /// 最优时间复杂度 —— O(n)
    fn bubble_with_flag(&mut self) {}

    /// 最优时间复杂度 —— 若已接近有序，则为 O(n)
    fn cocktail(&mut self) {}
}

impl Bubble for Vec<i32> {
    fn bubble(&mut self) {
        let len = self.len();
        for i in 0..len {
            for j in 0..len - i - 1 {
                if self[j] > self[j + 1] {
                    self.swap(j, j + 1);
                }
            }
        }
    }

    fn bubble_with_flag(&mut self) {}

    fn cocktail(&mut self) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bubble() {
        let mut v = vec![3, 1, 2, 4, 0, 5, 9, 7, 6, 8];
        v.bubble();
        assert_eq!(v, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_bubble_with_flag() {}

    #[test]
    fn test_cocktail() {}
}
