pub fn largest<T: PartialOrd>(num: &[T]) -> &T {
    let mut max = &num[0];
    for e in num {
        if *e > *max {
            max = e
        }
    }
    max
}

#[cfg(test)]
mod test {
    #[test]
    fn test1() {
        let v =
            vec!["hello".to_string(), "world".to_string(), "abc".to_string()];
        dbg!(super::largest(&v));
    }
}
