use std::vec;
mod largest;

fn main() {
    //let v = vec![10, 67, -1, 12, 4];
    let v = vec!["hello".to_string(), "world".to_string(), "abc".to_string()];
    dbg!(largest::largest(&v));
}
