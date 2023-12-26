mod model {
    pub mod test_demo;
}

use crate::model::test_demo::test_demo::test_demo;

fn main() {
    let tmp_test_demo = test_demo();
    println!("{}", tmp_test_demo);
}
