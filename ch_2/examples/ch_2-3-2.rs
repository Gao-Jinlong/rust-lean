fn main() {
    let x = 5;
    let y = &x; // y 是 x 的引用

    assert_eq!(5, x);
    assert_eq!(5, *y); // 解引用

    let s1 = String::from("hello");
    let len = calculate_length(&s1); // s1 的引用传递给函数，不会发生所有权转移
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    greeting(&mut s); // 可变引用传递给函数，可以修改 s，同时只能有一个可变引用，避免数据竞争
    println!("{}", s);

    // 可变引用与不可变引用不能同时存在
    let s = String::from("hello");
    let s1 = &s;
    let s2 = &s;
    // let s3 = &mut s; // panic

    println!("{}, {}", *s1, *s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn greeting(s: &mut String) {
    s.push_str(", world!");
}
