use crate::DATA;
use algorithms_rs::sort::Selection;

#[test]
fn test_selection_sort() {
    let data = DATA.clone();

    for (i, vec) in data.unsorted.iter().enumerate() {
        let mut sorted_vec = vec.clone();
        sorted_vec.selection_sort();
        assert_eq!(sorted_vec, data.sorted[i]);
    }
}
