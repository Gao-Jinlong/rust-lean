pub fn main() {
    // rust 的错误处理分为两类，一类是 panic，一类是 Result<T, E>
    // panic 会导致程序退出，而 Result<T, E> 则不会
    panic!("crash and burn");

    // 通过添加环境变量可以查看 panic 时的调用栈
    // RUST_BACKTRACE=1 cargo run
    // 为了获取这些信息，必须启用 debug 标识
    // 不使用 --release 参数运行 cargo build 和 cargo run 时 debug 标识会默认启用
    let v = vec![1, 2, 3];
    v[99];
}
