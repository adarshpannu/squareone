use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn test() {
    let mtx = Arc::new(Mutex::new(9));
    let mtx_clone = Arc::clone(&mtx);

    let child_thread = thread::spawn(move || {
        let mut val = mtx_clone.lock().unwrap();
        *val += 10;
        println!("child thread, val = {}", val);
    });

    thread::sleep(Duration::from_secs(1));
    let val = mtx.lock().unwrap();
    println!("main thread, val = {}", val);

    child_thread.join();
}
