fn main() {
    // 该方式创建的字符串存储在堆上
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{s}");

    // 一个变量在同一时间只能有一个所有者
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // 这是没有问题的，因为字面量所占用的空间是固定的，因此会被直接存入栈中，而 y = x 会将 x 的值复制一份到 y 中

    // 但是对于堆上的数据，就不一样了
    let s1 = String::from("hello");
    let s2 = s1;
    // s2 和 s1 指向同一个堆内存，当 s1 被释放时，s2 仍然指向这块内存，这样就会造成内存的二次释放，这是不允许的，因此 rust 会自动将 s1 的所有权转移到 s2 中
    // println!("{}, world!", s1); 这里会报错，因为 s1 的所有权已经被转移到了 s2 中，s1 无法再使用
    println!("{}, world!", s2); // 这里是没有问题的，因为 s2 已经拥有了 s1 的所有权，这个过程称为 move
                                // 当 s2 所在的作用域结束时，s2 会调用 drop 函数，释放 s2 所占用的空间，这个过程称为 drop
    let s1 = String::from("hello");
    let s2 = s1.clone(); // clone 函数会将堆上的数据复制一份
    println!("s1 = {}, s2 = {}", s1, s2);

    // 如果一个类型实现了 Copy trait 则可以直接复制，而不是转移所有权
    // 所有整型、浮点型、布尔型、字符型、元组类型都实现了 Copy trait
    // Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait, 如果我们对其值离开作用域时需要特殊处理的类型使用 Copy 注解, 将会出现一个编译时错误

    let s = String::from("hello");

    println!("{s}");

    takes_ownership(s);

    // println!("{s}"); 这里会报错，因为 s 的所有权已经被转移到 takes_ownership 函数中了
    // 当我们想要保留 s 的 ownership 时，可以使用 clone
    // takes_ownership(s.clone());

    let x = 5;
    makes_copy(x);
    println!("{x}");

    let s1 = gives_ownership(); // gives_ownership 将返回值移动给 s1
    let s2 = String::from("hello"); // s2 被创建
    let s3 = takes_and_gives_back(s2); // s2 被移动到 takes_and_gives_back 中, 它也将返回值移动给 s3

    println!("{s1}, {s3}");

    // let s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);
    // println!("The length of {s2} is {len}");

    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 通过引用传递参数，这样就不会转移所有权，引用的变量默认是不可被修改的
    println!("The length of {s1} is {len}");

    let mut s = String::from("hello");
    change(&mut s);
    let r1 = &mut s;
    // let r2 = &mut s; // 这里会报错，因为在同一作用域中，只能有一个可变引用
    // 可变引用不能同时存在多个，因为这样会造成数据竞争。

    println!("mutable reference s = {r1}");

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // 不可变引用可以存在多个，但是不可变引用不能和可变引用同时存在

    println!("{} and {}", r1, r2);
    // 引用声明的作用于从声明的地方开始一直持续到最后一次使用为止。

    let r3 = &mut s; // 这里是可以的，因为 r1 和 r2 的作用域已经结束了
    println!("r3 = {}", r3);

    let s = dangle();
    println!("dangle s = {}", s);

    // slice
    let s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); 这里会报错，因为 first_word 返回的是一个字符串的切片，而 s.clear() 会清空整个字符串，这样就会导致 word 指向的字符串的内容发生变化，这是不允许的
    println!("the first word is: {word}")
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn dangle() -> String {
    let s = String::from("hello"); // 在别的有指针的语言中很容易通过释放内存时保留一个指向它的指针而错误地生成一个 垂悬指针（dangling pointer)
                                   // 所谓垂悬指针是其指向的内存可能已经被分配给其它持有者。
                                   // Rust 中编译器确保引用永远也不会变成垂悬状态。
                                   // 编译器确保数据不会在其引用之前离开作用域。

    // &s // 返回字符串 s 的引用
    // 这里会报错，因为 s 在离开作用域时会被释放，因此返回的引用就是一个垂悬指针
    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    // 通过 slice 方法获取字符串的引用切片
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
