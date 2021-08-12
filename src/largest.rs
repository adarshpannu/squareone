
pub fn largest<T: PartialOrd>(num: &[T]) -> &T {
    let mut max = &num[0];
    for e in num {
        if *e > *max {
            max = e
        }
    }
    max
}
