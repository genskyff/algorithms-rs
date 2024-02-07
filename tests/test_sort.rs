mod data;
mod sort;
use data::TestData;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DATA: TestData = TestData::new();
}
