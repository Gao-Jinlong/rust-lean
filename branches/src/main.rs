use std::io;

fn main() {
    let mut number = 0;

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

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
