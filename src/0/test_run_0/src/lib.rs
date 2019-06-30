// `cargo test -- --nocapture`コマンドを実行すると表示される
fn print() { println!("***** Function!! *****"); }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        print();
        assert_eq!(2 + 2, 4);
    }
}
