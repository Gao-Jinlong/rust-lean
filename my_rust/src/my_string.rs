pub fn main() {
    // String 被实现为一个带有一些额外保证、限制和功能的字节 vector 封装
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // 拼接字符串
    let mut s = String::from("foo");

    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // + 可以用来拼接字符串 + 运算符使用了 add 这个函数， s1 被移动，不能继续使用
    println!("s3 is {}, s1 was borrowed, s2 is {}", s3, s2);
    // fn add(self, s: &str) -> String { add 函数的函数签名（替换了泛型），可以看到只能将 &str 和 String 相加，而不能将两个 String 相加
    // &s2 的类型是 &String 而不是 &str，但依然可以在 add 调用中使用 &s2 是因为 &String 可以被强转(coerced)为 &str

    // 对于更复杂的字符串链接，可以使用 format! 宏
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("the result of format! {}", s); // format! 宏使用引用，因此不会获取任何参数的所有权

    // rust 不支持通过索引访问字符串，是因为部分 u8 字符是通过两个字节来表示的，而访问单个位置不会得到一个正确的返回，因此为了避免这个问题，rust 会拒绝编译索引访问字符串。
    // 但是当你真的想要使用索引访问字符串时，rust 要求更明确一些，可以使用 slice，相比使用 [] 和单个值的索引，可以使用 [] 和一个 range 来创建含特定字节的字符串 slice
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    // 这会明确的访问前四个字节也就是前两个字符
    // 如果获取 &hello[0..1] 会使程序 panic，就像访问 vector 中的无效索引时一样

    // 遍历字符串，可以使用 chars 确保返回字符而不是字节
    for c in "Зд".chars() {
        println!("access string by chars {}", c)
    }
    // 使用 bytes 则可以明确的返回字节
    for b in "Зд".bytes() {
        println!("access string by bytes {}", b);
    }
}
