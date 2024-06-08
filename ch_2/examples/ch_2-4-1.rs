fn main() {
    let _s = "你好";
    // 中文在 UTF-8 中占 3 个字节，所以这里会 panic
    // let _s1 = &_s[0..2]; // byte index 2 is not a char boundary; it is inside '你' (bytes 0..3) of `你好`

    let mut s = String::from("hello world");
    let word = first_word(&s);

    println!("the first word is: {}", word);

    s.clear(); // 这会清空字符串，使其长度为 0

    // println!("the first word is: {}", word); // panic , 因为 word 是一个指向 s 的引用，而 s 已经被清空了

    // String 和 &str 转换
    // &str -> String
    let _s = String::from("hello, world!");
    let _s = "hello, world!".to_string();

    // String -> &str
    let mut s = String::from("ginlon");
    say_hello(&s); // 传递 String 的引用给 say_hello，say_hello
    s.clear();
    s.push_str("Ginlon");
    say_hello(&s);

    // 通过索引访问 String 会 panic
    let _s = String::from("hello");
    // print!("{}", _s[0]); // error: the trait `Index<_>` is not implemented for `String`
    // 这是因为 String 底层的存储格式是 [u8]
    // 而每个字母使用的是 UTF-8 编码，所以一个字母可能占用多个字节
    // 每个中文字符占用 3 个字节，这种情况下直接使用索引取不到正确的字符
    // 还有一个原因：我们对索引操作的期望是 O(1) 的时间复杂度，但是对于 String 来说无法保证这点，Rust 需要遍历字符串来定位合法字符

    // String Api
    let mut s = String::from("hello");
    s.push_str(" rust!");
    print!("push_str: {}\n", s);

    s.insert(5, ',');
    s.insert_str(6, " the great");
    print!("insert: {}\n", s);

    let s: String = s.replace("rust", "world");
    print!("replace: {}\n", s);

    let s = "I like rust. Learning rust is my favorite!";
    let s = s.replacen("rust", "RUST", 1); // 只替换一个
    print!("replacen: {}\n", s);

    let mut s = String::from("I like rust. Learning rust is my favorite!");
    s.replace_range(7..8, "R");
    print!("replace_range: {}\n", s);

    let mut s = String::from("hello");

    let discard = s.pop().unwrap();
    print!("pop: {} string: {}\n", discard, s);

    s.remove(s.len() - 1); // 删除指定位置的字符
    print!("remove: {}\n", s);

    let mut s = String::from("hello, world!");
    s.truncate(s.find(',').unwrap()); // 删除指定位置之后的字符
    print!("truncate: {}\n", s);

    s.clear();

    let s1 = String::from("hello");
    let s2 = String::from(" world!");

    let s3 = s1 + &s2; // s1 的所有权被转移，所以 s1 不能再使用

    // println!("s2: {}", s1); //  panic: value borrowed here after move

    print!("+=: {} s2 = {}\n", s3, s2);

    let s1 = "hello";
    let s2 = "world!";

    let s = format!("{} {}", s1, s2); // format! 宏类似于 println!，但是不会打印，而是返回一个 String
    print!("format: {}\n", s);

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";

    println!("{}", long_string);

    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}"; // r"..." 表示原始字符串，不会转义
    println!("{}", raw_str);

    let raw_str = r#"And then I said: "There is no escape!""#; // r#"..."# 如果字符串中包含 "，可以使用这种方式
    println!("{}", raw_str);
    let raw_str = r###"No escapes here"# "##"either: \x3F \u{211D}"###; // r###"..."### 可以使用多个 # 来消除歧义
    println!("{}", raw_str);

    // 操作 UTF-8 字符串
    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }

    for c in "नमस्ते".bytes() {
        print!("{} ", c);
    }
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

fn say_hello(s: &str) {
    println!("Hello, {}!", s)
}
