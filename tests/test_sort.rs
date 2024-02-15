mod data;
mod sort;
use data::TestData;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DATA: TestData = TestData::new();
}

#[test]
#[ignore]
fn test_data() {
    println!("unsorted:\n{:?}", &DATA.unsorted[0..6]);
    println!("sorted:\n{:?}", &DATA.sorted[0..6]);
}
