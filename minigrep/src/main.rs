use std::env;
use std::process;
use std::fs;

use lib::Config;

mod lib;

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

fn main(){

    let args: Vac<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}",config.query);
    println!("In file {}",config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
} 

struct Config{
    query:String,
    file_path:String,
}

fn parse_config(args:&[String]) -> (&str, &str){
    // TODO: 目前使用 clone() 方法来复制字符串，可以避免所有权问题，但并不是性能的最优解
    let query = &args[1].clone();
    let file_path =&args[2].clone();

    Config{ query, file_path}
}