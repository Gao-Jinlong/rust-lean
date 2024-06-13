use std::vec;

/// 存储不同类型的元素的列表

/// 通过枚举类型来存储不同类型的元素
enum IpAddr {
    V4(String),
    V6(String),
}

/// 通过 trait 实现存储不同类型
trait IpAddrTrait {
    fn display(&self);
}
struct V4(String);
impl IpAddrTrait for V4 {
    fn display(&self) {
        println!("V4: {}", self.0);
    }
}
struct V6(String);
impl IpAddrTrait for V6 {
    fn display(&self) {
        println!("V6: {}", self.0);
    }
}

/// 特征对象数组要比枚举数组常见的多，因为特征对象更灵活，且编译器对枚举的限制较多，且无法动态增加类型

fn main() {
    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    for ip in v {
        show_addr(ip);
    }

    let v: Vec<Box<dyn IpAddrTrait>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }

    // 动态数组容量
    let mut v = Vec::with_capacity(10); // 预分配10个元素的空间
    v.extend([1, 2, 3]); // 附加数据
    println!("Vector 长度是：{}; 容量是：{}", v.len(), v.capacity());

    v.reserve(100); // 调整 v 的容量为 v.len() + 100
    println!("Vector 长度是：{}; 容量是：{}", v.len(), v.capacity());

    v.shrink_to_fit(); // 释放多余的容量
    println!("Vector 长度是：{}; 容量是：{}", v.len(), v.capacity());

    v.retain(|x| x % 2 == 0); // 保留偶数
    println!("Vector 长度是：{}; 容量是：{}", v.len(), v.capacity());

    let mut v = vec![1, 2, 3, 4, 5];
    let m: Vec<_> = v.drain(1..=3).collect();
    print!("v: {:?}, m: {:?}", v, m);

    // 排序方法
    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    // vec.sort(); // panic 浮点数中有一个 NaN 无法进行比较，因此没有实现 Ord trait 但实现了 PartialOrd trait
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", vec);

    #[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn new(name: &str, age: u32) -> Self {
            Self {
                name: name.to_string(),
                age,
            }
        }
    }

    let mut people = vec![
        Person::new("John", 23),
        Person::new("Alice", 30),
        Person::new("Bob", 20),
    ];

    people.sort_unstable_by(|a, b| b.age.cmp(&a.age));
    println!("{:?}", people);

    let mut people = vec![
        Person::new("John", 23),
        Person::new("Alice", 30),
        Person::new("Bob", 20),
    ];
    // 通过 derive 实现的默认 Ord trait 会依次比较结构体中的每个字段来进行排序
    people.sort_unstable();
    println!("{:?}", people);
}

fn show_addr(ip: IpAddr) {
    match ip {
        IpAddr::V4(addr) => println!("V4: {}", addr),
        IpAddr::V6(addr) => println!("V6: {}", addr),
    }
}
