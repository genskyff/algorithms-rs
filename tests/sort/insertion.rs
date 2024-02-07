use crate::DATA;
use algorithms_rs::sort::Insertion;

#[test]
fn test_insertion_sort() {
    let data = DATA.clone();

    for (i, vec) in data.unsorted.iter().enumerate() {
        let mut sorted_vec = vec.clone();
        sorted_vec.insertion_sort();
        assert_eq!(sorted_vec, data.sorted[i]);
    }
}

#[test]
fn test_binary_insertion_sort() {
    let data = DATA.clone();

    for (i, vec) in data.unsorted.iter().enumerate() {
        let mut sorted_vec = vec.clone();
        sorted_vec.binary_insertion_sort();
        assert_eq!(sorted_vec, data.sorted[i]);
    }
}

#[test]
fn test_shell_sort() {
    let data = DATA.clone();

    for (i, vec) in data.unsorted.iter().enumerate() {
        let mut sorted_vec = vec.clone();
        sorted_vec.shell_sort();
        assert_eq!(sorted_vec, data.sorted[i]);
    }
}
