use std::error::Error;

mod guess_game;
mod my_map;
mod my_panic;
mod my_result;
mod my_string;
mod my_vector;

fn main() -> Result<(), Box<dyn Error>> {
    // main 函数可以返回 Result 类型，从而支持在 main 中使用 ? 运算符
    // Box<dyn Error> 是一个 trait 对象，可以暂时理解为任意类型错误
    println!("Hello, world!");

    my_vector::main();

    println!("----------------");

    my_string::main();

    println!("----------------");

    my_map::main();

    println!("----------------");

    // my_panic::main();

    println!("----------------");

    my_result::main();

    println!("----------------");

    guess_game::main();

    Ok(())
}
