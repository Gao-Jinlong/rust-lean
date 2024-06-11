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
}
