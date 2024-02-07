use crate::DATA;
use algorithms_rs::sort::Bubble;

#[test]
fn test_bubble_sort() {
    let data = DATA.clone();

    for (i, vec) in data.unsorted.iter().enumerate() {
        let mut sorted_vec = vec.clone();
        sorted_vec.bubble_sort();
        assert_eq!(sorted_vec, data.sorted[i]);
    }
}

#[test]
pub fn test_cocktail_sort() {
    let data = DATA.clone();

    for (i, vec) in data.unsorted.iter().enumerate() {
        let mut sorted_vec = vec.clone();
        sorted_vec.cocktail_sort();
        assert_eq!(sorted_vec, data.sorted[i]);
    }
}
