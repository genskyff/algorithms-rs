use algorithms_rs::sort::Insertion;
mod data;
use data::*;

#[test]
fn test_insertion_sort() {
    let mut vec = Vec::from(DATA);
    let mut arr = DATA;
    let slice = &mut DATA.clone()[..];

    vec.insertion_sort();
    arr.insertion_sort();
    slice.insertion_sort();

    assert_eq!(vec, DATA_SORTED);
    assert_eq!(arr, DATA_SORTED);
    assert_eq!(slice, DATA_SORTED);
}
