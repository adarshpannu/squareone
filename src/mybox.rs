use std::ops::Deref;

#[test]
fn test_main() {
    let x = 10;
    let px = &x;
    let bx = MyBox::new(20);
    let star_bx = bx.deref();

    let star_bx = *bx;

    
    println!("*px = {}", *px);
    println!("*bx = {}", *bx);
}

struct MyBox<T> {
    t: T
}

impl<T> MyBox<T> {
    fn new(t: T) -> Self {
        MyBox { t }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

