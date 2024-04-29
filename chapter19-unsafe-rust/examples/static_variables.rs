static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn main() {
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        // 多线程访问静态变量时会导致数据竞争
        println!("COUNTER: {}", COUNTER);
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
