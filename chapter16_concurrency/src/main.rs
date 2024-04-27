use std::sync::mpsc;
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

    println!("===========================");
    // 不要通过共享内存来通讯；而是通过通讯来共享内存。

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");

        tx.send(val).unwrap();

        // println!("val is {}", val); // 这里会报错，因为val的所有权已经转移给了tx
    });

    let received = rx.recv().unwrap(); // recv方法会阻塞主线程，直到接收到消息
    println!("Got: {}", received);

    println!("===========================");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // thread::spawn(move || {
    //     for received in rx {
    //         println!("thread_1: Got: {}", received);
    //     }
    // });

    for received in rx {
        println!("Got: {}", received);
    }

    println!("===========================");
    // 多生产者，单消费者

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("thread_2: Got: {}", received);
    }
}
