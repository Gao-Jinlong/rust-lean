use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// 创建线程池
    ///
    /// 线程池中线程的数量
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver)); // Arc 使得多个 worker 拥有接收端， Mutex 确保一次只有一个 worker 能从接收端得到任务

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            };
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv(); // let 语句结束时等号右侧使用的临时值都会被立即丢弃，所以这里 lock 返回的 MutexGuard 会在这里被立即释放

            match message {
                Ok(job) => {
                    println!("Worker {id} get a job; executing");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} receive a shutdown signal");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }

    // 如果使用 while let
    // fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    //     let thread = thread::spawn(move || {
    //         while let Ok(job) = receiver.lock().unwrap().recv() {
    //             // while let (if let 和 match) 直到相关的代码块结束都不会丢弃临时值，因此这个锁会一直被持有，其他的 worker 无法获取锁
    //             println!("Worker {id} get a job; executing");

    //             job();
    //         }
    //     });

    //     Worker { id, thread }
    // }
}
