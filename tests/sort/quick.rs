use crate::DATA;
use algorithms_rs::sort::Quick;

#[test]
fn test_quick_sort() {
    let data = DATA.clone();

    for (i, vec) in data.unsorted.iter().enumerate() {
        let mut sorted_vec = vec.clone();
        sorted_vec.quick_sort();
        assert_eq!(sorted_vec, data.sorted[i]);
    }
}
