use algorithms_rs::sort::Bubble;
mod data;
use data::*;

#[test]
fn test_bubble_sort() {
    let mut vec = Vec::from(DATA);
    let mut arr = DATA;
    let slice = &mut DATA.clone()[..];

    vec.bubble_sort();
    arr.bubble_sort();
    slice.bubble_sort();

    assert_eq!(vec, DATA_SORTED);
    assert_eq!(arr, DATA_SORTED);
    assert_eq!(slice, DATA_SORTED);
}

#[test]
fn test_cocktail_sort() {
    let mut vec = Vec::from(DATA);
    let mut arr = DATA;
    let slice = &mut DATA.clone()[..];

    vec.cocktail_sort();
    arr.cocktail_sort();
    slice.cocktail_sort();

    assert_eq!(vec, DATA_SORTED);
    assert_eq!(arr, DATA_SORTED);
    assert_eq!(slice, DATA_SORTED);
}
