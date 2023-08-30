enum IpAddrKind{
    V4(u8,u8,u8,u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> &Self{
        // method body would be defined here
        dbg!(self)
    }
}

fn main() {
    let four = IpAddrKind::V4(127,0,0,1);
    let six = IpAddrKind::V6(String::from("::1"));

    route(four);
    route(six);

    let m = Message::Write(String::from("hello"));
    m.call();
    let m = Message::Quit;
    m.call();
    let m = Message::Move{x:1, y:2};
    println!("{:?}", &m);
    m.call();
    let m = Message::ChangeColor(1,2,3);
    m.call();

    // rust 标准库实现了 Option<T>，用于编码可能为空的值
    // enum Option<T>{
    //     None,
    //     Some(T),
    // }
    let some_number = Some(5); 
    let some_char = Some('e');

    let absent_number:Option<i32> = None;

    let x:i8 = 5;
    let y:Option<i8> = Some(5);

    // let sum = x + y; // Option 的意义就在于不允许像使用非空值那样使用可能为空的值
}

fn route(ip_kind:IpAddrKind){}