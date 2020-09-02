use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawn 线程 {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!(" main 线程 {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
