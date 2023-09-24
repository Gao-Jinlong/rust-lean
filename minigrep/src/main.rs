use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}"); // 将错误输出到 stderr 而不是 stdout，使用 cargo run > output.txt 将正常输出写入文件，而错误信息仍然会显示在终端上
        process::exit(1);
    });

    // 使用 if 来检查 run 是否返回一个 Err 值
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
