// 孤儿规则（orphan rule） / 相干性（coherence）
// 如果想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的
// 如无法为 String 类型实现 Display 特征，因为 String 和 Display 都是在标准库中定义的

use std::fmt::Display;

// panic
// impl Display for String {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", self)
//     }
// }

pub trait Summary {
    fn summarize_author(&self) -> String;

    // 默认实现
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}
pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}
impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}
pub struct Weibo {
    pub username: String,
    pub content: String,
}
impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{} by {}", self.content, self.summarize_author())
    }
}

// 使用特征作为函数参数
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// impl Trait 只是一个语法糖
// 完整写法
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// 多个 Trait
pub fn notify3(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}
// where 从句
pub fn notify4<T>(item: &T)
where
    T: Summary + Display,
{
    println!("Breaking news! {}", item.summarize());
}

// 使用特征约束有条件的实现方法或特征
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
// 只有实现了 Display 和 PartialOrd 特征的类型才能调用 cmp_display 方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 函数返回 impl Trait
// 这种方式只能返回一个具体类型，不能返回两个不同的类型
// 可以使用枚举合并多个类型，但这要求在编写代码时就需要知道所有可能的类型
// 可以使用 特征对象 来解决动态不同类型的问题，见 ch_2-8-3.rs
fn returns_summarize() -> impl Summary {
    Weibo {
        username: String::from("Rust"),
        content: String::from("Rust 是一门系统编程语言"),
    }
}

fn main() {
    let post = Post {
        title: "Rust 语言简介".to_string(),
        author: "Rust".to_string(),
        content: "Rust 是一门系统编程语言".to_string(),
    };
    let weibo = Weibo {
        username: "Rust".to_string(),
        content: "Rust 是一门系统编程语言".to_string(),
    };
    println!("{}", post.summarize());
    println!("{}", weibo.summarize());

    notify(&post);
    notify(&weibo);

    let pair = Pair::new(1, 2);
    pair.cmp_display();

    let s = returns_summarize();
    println!("{}", s.summarize());

    // TryInto
    // 使用 trait 方法时，需要将特征引入当前作用域
    // TryInto 已经通过 std::prelude 模块引入，所以可以直接使用
    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than one hundred.")
    }
}
