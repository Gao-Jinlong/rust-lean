use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

pub fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // unwrap
    // 如果 File::open 返回 Ok 值，unwrap 会返回 Ok 中的值，如果它返回 Err 值，unwrap 会为我们调用 panic! 宏。
    let greeting_file = File::open("hello.txt").unwrap();

    // expect 可以自定义 panic 时的错误信息
    let greeting_file =
        File::open("hello.txt").expect("hello2.txt should be include in this project");

    read_username_from_file();
    read_username_from_file_shortest();

    let s = last_char_of_first_line("hello\nworld");
    println!("last char of first line is {:?}", s);
    let s = last_char_of_first_line("");
    println!("last char of first line is {:?}", s);
}

// 传播（propagating）错误
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello2.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // 使用 return 提前结束函数
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// 使用 ? 运算符，简化上面代码
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello2.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// 使用 fs::read_to_string 一行代码实现
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    // 打开一个文件，新建一个字符串，读取文件内容到字符串中，返回字符串
    fs::read_to_string("hello2.txt")
}

/*
match 表达式与 ? 运算符所做的有一点不同，? 运算符所使用的错误值被传递给了 from 函数，它定义于标准库的 From trait 中，其用来将错误从一种类型转换为另一种类型。
当 ? 运算符调用 from 函数时，其所需的错误类型会被转换为定义为当前函数返回的错误类型。
这在将错误传递给调用者时非常有用，但是在函数内部仍然希望使用 match 表达式来处理错误的场景也很常见。
? 运算符只能被用于返回 Result 或其他实现了 FromResidual 的函数，也可以用于返回 Option 的函数。
*/

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
