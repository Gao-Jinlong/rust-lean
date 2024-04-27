use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();

        *num = 6;
    }

    println!("m = {:?}", m);

    // Mutex<T> 是一个智能指针，lock方法返回一个叫做 MutexGuard 的智能指针
    // MutexGuard 实现了 Deref 和 Drop trait
    // Deref trait 允许我们通过解引用运算符访问 MutexGuard 中的数据
    // Drop trait 允许我们在 MutexGuard 离开作用域时释放锁

    println!("===========================");

    // let counter = Rc::new(Mutex::new(0)); // Rc<T> 并不能安全的在线程间共享。
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // 阻塞主线程，直到子线程结束
    }

    println!("Result: {}", *counter.lock().unwrap());
}
