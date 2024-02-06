mod data;
use algorithms_rs::sort::Insertion;
use data::TestData;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DATA: TestData = TestData::new();
}

#[test]
fn test_insertion_sort() {
    for (i, vec) in DATA.unsorted.iter().enumerate() {
        let mut sorted_vec = vec.clone();
        sorted_vec.insertion_sort();
        assert_eq!(sorted_vec, DATA.sorted[i]);
    }
}

#[test]
fn test_binary_insertion_sort() {
    for (i, vec) in DATA.unsorted.iter().enumerate() {
        let mut sorted_vec = vec.clone();
        sorted_vec.binary_insertion_sort();
        assert_eq!(sorted_vec, DATA.sorted[i]);
    }
}

#[test]
fn test_shell_sort() {
    for (i, vec) in DATA.unsorted.iter().enumerate() {
        let mut sorted_vec = vec.clone();
        sorted_vec.shell_sort();
        assert_eq!(sorted_vec, DATA.sorted[i]);
    }
}
