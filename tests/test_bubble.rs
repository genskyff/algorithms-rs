use algorithms_rs::sort::Bubble;
mod data;
use data::*;

#[test]
fn test_bubble_i() {
    let mut vec = Vec::from(DATA_I);
    let mut arr = DATA_I;
    let slice = &mut DATA_I.clone()[..];

    vec.bubble();
    arr.bubble();
    slice.bubble();

    assert_eq!(vec, DATA_I_SORTED);
    assert_eq!(arr, DATA_I_SORTED);
    assert_eq!(slice, DATA_I_SORTED);
}

#[test]
fn test_bubble_f() {
    let mut vec = Vec::from(DATA_F);
    let mut arr = DATA_F;
    let slice = &mut DATA_F.clone()[..];

    vec.bubble();
    arr.bubble();
    slice.bubble();

    assert_eq!(vec, DATA_F_SORTED);
    assert_eq!(arr, DATA_F_SORTED);
    assert_eq!(slice, DATA_F_SORTED);
}

#[test]
fn test_bubble_flag_i() {}

#[test]
fn test_bubble_flag_f() {}

#[test]
fn test_bubble_cocktail_i() {}

#[test]
fn test_bubble_cocktail_f() {}
