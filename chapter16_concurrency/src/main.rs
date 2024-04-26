use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap(); // 会阻塞主线程，直到子线程结束

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // 阻塞主线程，直到子线程结束

    println!("===========================");

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // 使用move关键字，将v的所有权转移给子线程,避免在子进程未结束前 v 被销毁
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
