mod data;
use algorithms_rs::sort::Selection;
use data::TestData;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DATA: TestData = TestData::new();
}

#[test]
fn test_selection_sort() {
    for (i, vec) in DATA.unsorted.iter().enumerate() {
        let mut sorted_vec = vec.clone();
        sorted_vec.selection_sort();
        assert_eq!(sorted_vec, DATA.sorted[i]);
    }
}
