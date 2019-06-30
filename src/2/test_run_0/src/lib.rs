pub fn add_two(a: i32) -> i32 {
    a + 2
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    #[ignore]
    fn expensive_test() {
        // 実行に1時間かかるコード
        // code that takes an hour to run
    }
}

