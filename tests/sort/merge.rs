use crate::DATA;
use algorithms_rs::sort::Merge;

#[test]
fn test_merge_sort_recu() {
    let data = DATA.clone();

    for (i, vec) in data.unsorted.iter().enumerate() {
        let mut sorted_vec = vec.clone();
        sorted_vec.merge_sort_recu();
        assert_eq!(sorted_vec, data.sorted[i]);
    }
}

#[test]
fn test_merge_sort_iter() {
    let data = DATA.clone();

    for (i, vec) in data.unsorted.iter().enumerate() {
        let mut sorted_vec = vec.clone();
        sorted_vec.merge_sort_iter();
        assert_eq!(sorted_vec, data.sorted[i]);
    }
}
