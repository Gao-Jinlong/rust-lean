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
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin:Coin) -> u8{
    match coin{
        Coin::Penny => {
            println!("Luck penny!");
            1}
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state)=>{
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None=>None,
        Some(i) => Some(i+1),
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
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?} {:?} {:?}", five, six, none);

    // other 匹配所有其他情况
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // _ 忽略值
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // 明确忽略其他值
    }

    // 忽略模式
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ =>(),
    }

    // if let 针对单一情况处理，不需要像 match 一样穷尽所有值
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("Max is {}", max);
    }

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => print!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    // 等同于
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count +=1;
    }
    
}

fn route(ip_kind:IpAddrKind){}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}