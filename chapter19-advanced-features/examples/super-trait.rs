use std::fmt;
use std::ops::Add;
use std::ops::Deref;

pub trait Iterator {
    type Item; // 关联类型
    fn next(&mut self) -> Option<Self::Item>;
}

pub struct Counter {
    count: u32,
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// 泛型和关联类型的区别在于，泛型可以实现多个不同的类型，而关联类型只能实现一个具体的类型
pub trait IteratorWithGeneric<T> {
    fn next(&mut self) -> Option<T>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}

// ======================
// 完全限定语法与消歧义 (Fully Qualified Syntax for Disambiguation)
// 同一类型可以实现多个相同的 trait，但是这样会导致冲突，可以使用完全限定语法来消除歧义
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// ===================

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// =====================
// super trait
trait OutlinePrint: fmt::Display {
    // 指定需要实现 Display trait
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// impl OutlinePrint for Point {} // 未实现 Display 时会报错 `Point` cannot be formatted with the default formatter

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}
// 实现了 Display 后就可以实现 OutlinePrint
impl OutlinePrint for Point {}

// ===============================================
// newtype 模式
// rust 的孤儿规则 (orphan rule) 限制了 trait 的使用，只要 trait 或类型相对于当前的 crate 是本地的，我们才可以为该类型实现 trait
// 但是可以使用 newtype 模式来绕过这个限制
struct Wrapper(Vec<String>); // 使用一个没有命名字段的元组结构体来创建一个新类型

// newtype 模式的缺陷是，因为 Wrapper 是一个新类型，所以它没有 Vec 的方法，需要手动实现
// 为该封装类型实现 Deref trait 并返回内部类型是一种解决方案
impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    person.fly(); // 默认调用直接实现在类型上的方法

    // 通过 self 的类型 rust 可以判断出该使用哪个 trait 中的方法
    Pilot::fly(&person);
    Wizard::fly(&person);
    Human::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name()); // 这里没有 self 参数也就无法区分想要调用的 trait

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // 完全限定语法，提供了类型注解

    let point = Point { x: 1, y: 2 };
    point.outline_print();
    println!("{}", point);

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    println!("w.len = {}", w.len());
}
