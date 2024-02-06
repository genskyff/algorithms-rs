mod data;
use algorithms_rs::sort::Bubble;
use data::TestData;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DATA: TestData = TestData::new();
}

#[test]
fn test_bubble_sort() {
    for (i, vec) in DATA.unsorted.iter().enumerate() {
        let mut sorted_vec = vec.clone();
        sorted_vec.bubble_sort();
        assert_eq!(sorted_vec, DATA.sorted[i]);
    }
}

#[test]
fn test_cocktail_sort() {
    for (i, vec) in DATA.unsorted.iter().enumerate() {
        let mut sorted_vec = vec.clone();
        sorted_vec.cocktail_sort();
        assert_eq!(sorted_vec, DATA.sorted[i]);
    }
}
