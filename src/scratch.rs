#[derive(Debug)]
struct Foo {
    value: i32,
}

#[derive(Debug)]
struct Bar2<'b> {
    x: &'b Foo,
    y: Box<Foo>,
}

impl<'b> Bar2<'b> {
    fn f(&mut self) -> &Foo {
        /*
        // replace self.y with an updated one
        self.y = Box::new(Foo {
            value: self.y.value + 1,
        });
        println!("self.y = {:?}", self.y);
        // then dereference what's in self.x
        println!("old self.x {:?}", self.x);
        */
        self.x = &*self.y;
        println!("new self.x {:?}", self.x);
        self.x
    }
}

#[test]
fn main() {
    let foo = Foo { value: 10 };
    let mut bar2 = Bar2 {
        x: &foo,
        y: Box::new(Foo { value: 20 }),
    }; // #1
    bar2.f(); // #2      'bar2' is borrowed as mutable
    //let z = bar2.f(); // #3 error: cannot borrow `bar2` as mutable more than once at a time [E0499]
}

fn test() {}
