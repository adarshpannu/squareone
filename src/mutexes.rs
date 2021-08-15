
use std::sync::Mutex;

#[test]
fn test() {
    let m = Mutex::new(99);

    let i = m.lock().unwrap();

    dbg!(&i);

    let j = m.try_lock();
    dbg!(&j);

}