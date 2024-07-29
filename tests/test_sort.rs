mod data;
mod sort;
use data::TestData;
use std::sync::LazyLock;

static DATA: LazyLock<TestData> = LazyLock::new(TestData::new);

#[test]
#[ignore]
fn test_data() {
    println!("unsorted:\n{:?}", &DATA.unsorted[0..6]);
    println!("sorted:\n{:?}", &DATA.sorted[0..6]);
}
