use add_one;

// 为了在顶层运行二进制 crate 可以通过 -p 参数和包名来运行
// cargo run -p adder
fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
