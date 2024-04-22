use std::env;
use std::process;

// use lib::Config;

// mod lib;

// fn main() {
//     let config = Config::build(env::args()).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {err}"); // 将错误输出到 stderr 而不是 stdout，使用 cargo run > output.txt 将正常输出写入文件，而错误信息仍然会显示在终端上
//         process::exit(1);
//     });

//     // 使用 if 来检查 run 是否返回一个 Err 值
//     if let Err(e) = lib::run(config) {
//         eprintln!("Application error: {e}");
//         process::exit(1);
//     };
// }

mod lib;

use lib::Config;

fn main() {
    // unwrap_or_else 用于处理 Result 的 Ok 和 Err 两种情况
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // run(config).unwrap_or_else(|err|{
    //     println!("Application error: {err}");
    //     process::exit(1);
    // })

    // if let 控制流 用于处理只关心一个分支的情况
    if let Err(e) = lib::run(config) {
        // eprintln! 宏类似 println!，但是输出到 stderr 而不是 stdout，使用 cargo run > output.txt 将正常输出写入文件，而错误信息仍然会显示在终端上
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
