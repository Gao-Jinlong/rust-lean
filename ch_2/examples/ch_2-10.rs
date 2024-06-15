use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 编译器无法知道 'a 和 'b 的关系，因此会报错
    // 由于 'a 是被引用的，因此 'a 的生命周期必须比 'b 长
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
    where
        'a: 'b, // 用于限定 'a 和 'b 的关系，'a 必须比 'b 长
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}
fn main() {
    // 'static 生命周期
    // 'static 生命周期是一个特殊的生命周期，它会存活于整个程序期间
    let s1: &str;
    {
        let s: &'static str = "I have a static lifetime.";
        println!("{}", s);

        // 离开作用域后虽然 s 被销毁，但是 "I have a static lifetime." 会一直存在
        s1 = s;
    }

    println!("{}", s1);
}

fn longest_with_an_announcement<'a, 'b, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
