use algorithms_rs::sort::Selection;
mod data;
use data::*;

#[test]
fn test_selection_sort_i() {
    let mut vec = Vec::from(DATA_I);
    let mut arr = DATA_I;
    let slice = &mut DATA_I.clone()[..];

    vec.selection_sort();
    arr.selection_sort();
    slice.selection_sort();

    assert_eq!(vec, DATA_I_SORTED);
    assert_eq!(arr, DATA_I_SORTED);
    assert_eq!(slice, DATA_I_SORTED);
}

#[test]
fn test_selection_sort_f() {
    let mut vec = Vec::from(DATA_F);
    let mut arr = DATA_F;
    let slice = &mut DATA_F.clone()[..];

    vec.selection_sort();
    arr.selection_sort();
    slice.selection_sort();

    assert_eq!(vec, DATA_F_SORTED);
    assert_eq!(arr, DATA_F_SORTED);
    assert_eq!(slice, DATA_F_SORTED);
}
