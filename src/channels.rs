use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn test() {
    let (sx, rx) = mpsc::channel::<i32>();

    let join_handle = thread::spawn(move || {
        for i in 1..=10 {
            thread::sleep(Duration::from_millis(1));
            println!("child thread sent: {}", i);
            sx.send(i);
        }
    });

    for val in rx {
        println!("main thread got {}", val);
    }

    join_handle.join();
}
