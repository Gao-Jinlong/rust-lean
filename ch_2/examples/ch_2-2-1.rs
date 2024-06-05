use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.0, im: -1.0 }; // re 复数的实部，im 复数的虚部
    let b = Complex::new(1.0, -1.0); // new 是 Complex 的一个关联函数
    let result = a + b; // 重载了 + 运算符

    println!("{} + {} = {}", result.re, result.im, result);
    println!("sqrt = {}", result.norm_sqr());
}
