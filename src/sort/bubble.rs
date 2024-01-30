//! - 分类 —— 基于比较的排序
//! - 数据结构 —— 数组
//! - 最差时间复杂度 —— O(n<sup>2</sup>)
//! - 平均时间复杂度 —— O(n<sup>2</sup>)
//! - 就地性 —— O(1)
//! - 稳定性 —— 稳定

pub trait Bubble {
    /// 最优时间复杂度 —— O(n<sup>2</sup>)
    fn bubble(&mut self);

    /// 效率优化 —— 使用 flag 来表示是否需要交换 <br>
    /// 最优时间复杂度 —— O(n)
    fn bubble_with_flag(&mut self);

    /// 最优时间复杂度 —— 若已接近有序，则为 O(n)
    fn cocktail(&mut self);
}

impl<T: PartialOrd> Bubble for [T] {
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

    fn bubble_with_flag(&mut self) {
        todo!()
    }

    fn cocktail(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sort::data::*;

    #[test]
    fn test_bubble_i() {
        let mut vec = Vec::from(DATA_I);
        let mut arr = DATA_I;
        let slice = &mut DATA_I.clone()[..];

        vec.bubble();
        arr.bubble();
        slice.bubble();

        assert_eq!(vec, DATA_I_SORTED);
        assert_eq!(arr, DATA_I_SORTED);
        assert_eq!(slice, DATA_I_SORTED);
    }

    #[test]
    fn test_bubble_f() {
        let mut vec = Vec::from(DATA_F);
        let mut arr = DATA_F;
        let slice = &mut DATA_F.clone()[..];

        vec.bubble();
        arr.bubble();
        slice.bubble();

        assert_eq!(vec, DATA_F_SORTED);
        assert_eq!(arr, DATA_F_SORTED);
        assert_eq!(slice, DATA_F_SORTED);
    }

    #[test]
    fn test_bubble_with_flag_i() {}

    #[test]
    fn test_bubble_with_flag_f() {}

    #[test]
    fn test_cocktail_i() {}

    #[test]
    fn test_cocktail_f() {}
}
