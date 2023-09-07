pub fn main() {
    let tweet = Tweet {
        username: String::from("Ginlon"),
        content: String::from("Hello, world!"),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("1 new tweet: {}", article.summarize());
    notify(&tweet);
    notify2(&tweet, &tweet);

    println!("----------------------");
}
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)") // trait 可以提供默认实现
    }

    fn summarize_author(&self) -> String;

    fn summarize2(&self) -> String {
        // trait 可以提供默认实现
        // 默认实现可以调用 trait 中的其他方法，即使这些方法没有默认实现，只要为调用者实现了这些方法即可
        format!("(Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// impl Summary for NewsArticle {}; // 为 NewsArticle 添加默认实现
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{},by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}：{}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// trait 作为参数
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// trait bound 语法，适合多个参数，可以限制多个参数使用同一个类型
pub fn notify2<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {},{}", item1.summarize(), item2.summarize());
}
