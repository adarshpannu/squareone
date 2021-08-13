#![allow(warnings)]
struct Counter {
    cur: u32,
    max: u32
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { cur: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.max {
            self.cur += 1;
            Some(self.cur - 1)
        } else {
            None
        }
    }
}

#[test] 
fn test() {
    let ctr = Counter::new(10);
    let iter = ctr;

    for e in iter {
        println!("elem: {}", e)
    }
    println!("Test")
}

