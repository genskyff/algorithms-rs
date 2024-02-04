use algorithms_rs::sort::Selection;
mod data;
use data::*;

#[test]
fn test_selection_sort() {
    let mut vec = Vec::from(DATA);
    let mut arr = DATA;
    let slice = &mut DATA.clone()[..];

    vec.selection_sort();
    arr.selection_sort();
    slice.selection_sort();

    assert_eq!(vec, DATA_SORTED);
    assert_eq!(arr, DATA_SORTED);
    assert_eq!(slice, DATA_SORTED);
}
