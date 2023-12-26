/*
这是 my_module 的模块说明
*/
pub mod test_demo {
    pub fn test_demo() -> String {
        let mut x = String::from("hello");
        println!("{}", x);
        return x;
    }
}