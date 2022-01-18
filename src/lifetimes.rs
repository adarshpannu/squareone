struct Foo<'a> {
    bar: &'a str
}

impl<'a> Foo<'a> {
    fn hello(&self) -> &str {
        self.bar
    }
    fn world(&'a self) -> &str {
        self.bar
    }
}

/*
fn getstr<'a>(s: String) -> &'a str {
    &s[..]
}
*/

#[test]
fn test() {

    let bar = &"abc".to_string()[..];

    let foo = Foo { bar: bar };

    let h = foo.hello();
    let w = foo.world();

}
