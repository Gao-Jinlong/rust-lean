use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // let list = Cons(1, Cons(2, Cons(3, Nil))); // rust 编译器无法计算出这个递归定义的类型的大小，因此会报错。

    // 使用 Box<T> 来解决这个问题
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // 解引用运算符 * 和 Deref trait
    let x = 5;
    let y = &x; // y 是一个指向 x 的引用

    assert_eq!(5, x);
    // assert_eq!(5, y); //  no implementation for `{integer} == &{integer}`
    assert_eq!(5, *y); // 解引用运算符 * 可以获取引用的值

    // 像引用一样使用 Box<T>
    let x = 5;
    let y = Box::new(x); // y 是一个指向 x 的 Box<T> 类型的智能指针

    assert_eq!(5, x);
    assert_eq!(5, *y); // 解引用运算符 * 可以获取 Box<T> 的值

    // ----------------------------------
    // 自定义智能指针
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // 实现 Deref trait 后才可以使用解引用运算符 *
                       // 实际上，*y 会被替换为 *(y.deref())，解引用运算符 * 被设计为先调用 deref 方法，然后再进行普通的解引用操作
                       // *(y.deref()) 中的 * 是普通的解引用操作，是为了获得引用值的所有权。

    // 强制类型转换
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // 传递 MyBox<String> 类型的值，而不是 &String 类型的值
               // rust 先通过 MyBox 的 deref 方法将 &MyBox<String> 转换为 &String，然后再通过标准库中 String 的 deref 将 &String 转换为 &str

    // 如果没有强制类型转换
    hello(&(*m)[..]);

    // ----------------------------------
    // rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换：
    // 当 T: Deref<Target=U> 时从 &T 到 &U。
    // 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
    // 当 T: Deref<Target=U> 时从 &mut T 到 &U。
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T; // 定义关联类型

    fn deref(&self) -> &Self::Target {
        &self.0 // 返回通过解引用运算符 * 获取的值
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
