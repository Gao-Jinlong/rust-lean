use std::io;

fn main() {
    let number: i32;

    println!("Please input a number: ");

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<i32>() {
                Ok(n) => {
                    number = n;
                    break;
                }
                Err(_) => {
                    println!("Please input a number: ");
                    continue;
                }
            },
            Err(_) => {
                println!("Please input a number: ");
                continue;
            }
        };
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible bt 4, 3 or 2")
    }

    let condition = true;
    let x = if condition { 5 } else { 6 }; // if else 分支表达式，返回类型必须一致
    println!("the value of x is {}", x);
}
