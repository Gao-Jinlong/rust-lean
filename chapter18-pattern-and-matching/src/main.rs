fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let 条件循环
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let (x, y, z) = (1, 2, 3); // let 语句实际上也是一种模式

    fn foo(x: i32) {
        // 函数参数也是一种模式
        println!("{}", x);
    }

    // ===================
    // 匹配字面值
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 匹配变量名
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // 这里的 y 是一个新的变量，而不是之前定义的 y，因此 Some(y) 会匹配任意 Some 值
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // 匹配多个模式
    let x = 1;

    match x {
        1 | 2 => println!("one ot two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 使用 ..= 匌匹配值的范围
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 解构并分解值
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // 解构枚举
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            // 匹配嵌套项
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            );
        }
    }

    // 解构结构体和元祖
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // 忽略模式中的值
    fn foo2(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo2(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            // 使用多处 _ 忽略值
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // 注意使用 _ 完全不会绑定值，而使用下划线开头的名称如 _x 仍会将值绑定到变量并转移所有权

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    if let Some(_s) = s {
        println!("found a string");
    }

    // println!("{:?}", s); // !panic

    // 使用 .. 忽略剩余部分
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }

    // 匹配守卫
    let num = Some(4);

    // 编译器不会尝试为包含匹配守卫的模式检查穷尽性
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // 使用匹配守卫解决变量覆盖的问题
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end x = {:?}, y = {y}", x);

    // @ 绑定
    enum Message2 {
        Hello { id: i32 },
    }

    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello {
            id: id_variable @ 3..=7, // 将匹配的值绑定到 id_variable 方便后续使用，这里也可以使用 id 作为变量名，为了演示使用不同的变量名
        } => println!("Found an id in range: {}", id_variable),
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message2::Hello { id } => println!("Found some other id: {}", id),
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
