use std::ops::Deref;

struct SmartPtr<T> {
    inner: T
}

impl<T> SmartPtr<T> {
    fn new(inner: T) -> Self {
        SmartPtr { inner }
    }
}

impl<T> Deref for SmartPtr<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[test]
fn test() {
    let sp = SmartPtr::new(10);
    println!("Hello: {}", *sp);
}
