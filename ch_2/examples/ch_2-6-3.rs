fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top)
    }

    // 匹配守卫
    let num = Some(4);

    match num {
        Some(x) if x < 5 => print!("less than five\n"),
        Some(x) => print!("number: {}\n", x),
        None => (),
    }

    // @ 绑定
    enum Message {
        Hello { id: u32 },
    }

    let message = Message::Hello { id: 19 };

    match message {
        Message::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range: {}", id) // 将原始的 id 绑定到当前作用域的新变量 id
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range") // 这里没有绑定 id，所以不能使用 id
        }
        Message::Hello { id } => {
            // 语法糖，等价于 Message::Hello { id: id }
            println!("Found some other id: {}", id)
        }
    }

    // 绑定解构
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p @ Point { x: px, y: py } = Point { x: 0, y: 7 }; // p 是 Point 的引用，px 和 py 是 Point 的字段
    print!("p: {:#?} px: {} py: {}\n", p, px, py);

    let Point { x: x1, y: y1, .. } = Point { x: 0, y: 7 };
    print!("x1: {} y1: {}\n", x1, y1);
}
