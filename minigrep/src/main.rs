use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parasing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}.", config.query);
    println!("In file {}.", config.file_path);

    // 使用 if 来检查 run 是否返回一个 Err 值
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}
