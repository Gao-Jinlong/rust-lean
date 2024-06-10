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
    fn summarize(&self) -> String;
}
pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}
pub struct Weibo {
    pub username: String,
    pub content: String,
}
impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
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
}
