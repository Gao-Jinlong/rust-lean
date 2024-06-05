fn main() {
    // rust 通过所有权机制避免了二次释放和悬空指针的问题
    // 变量离开作用域时，rust 会自动调用 drop 方法释放资源

    // 基本类型存储在栈上，赋值直接通过拷贝栈上的值完成，因此不会发生所有权转移
    let mut x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    x = 6;
    println!("x = {}, y = {}", x, y);

    let s: &str = "hello";
    let s2 = s;
    println!("s = {}, s2 = {}", s, s2);

    // String 类型存储在堆上，赋值会发生所有权转移
    let s = String::from("hello"); // 存储在堆上的字符串
    let mut s2 = s; // 所有权转移
                    // s.push_str(", world!"); // panic
    s2.push_str(", world!");
    println!("s2 = {}", s2);

    let s = String::from("hello");
    let s2 = s.clone(); // 深拷贝，复制堆上的数据从而保留原始数据的所有权
    println!("s = {}, s2 = {}", s, s2);

    // Copy trait
    // Copy trait 适用于基本类型，当类型实现了 Copy trait 时，赋值操作会发生拷贝而不是所有权转移
    // 可变引用不实现 Copy trait

    let s = String::from("hello");
    takes_ownership(s);
    // println!("s = {}", s); // panic

    let x = 5;
    makes_copy(x);
    println!("x = {}", x);
}
fn takes_ownership(s: String) {
    // s 的所有权从调用者转移到函数内部
    println!("takes_ownership: {}", s);
    // s 离开作用域时，调用 drop 方法释放资源
}

fn makes_copy(x: i32) {
    // 基础类型实现了 Copy trait，赋值操作会发生拷贝而不是所有权转移
    println!("makes_copy: {}", x);
}
