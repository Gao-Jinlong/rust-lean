fn main() {
    let _s = "你好";
    // 中文在 UTF-8 中占 3 个字节，所以这里会 panic
    // let _s1 = &_s[0..2]; // byte index 2 is not a char boundary; it is inside '你' (bytes 0..3) of `你好`

    let mut s = String::from("hello world");
    let word = first_word(&s);

    println!("the first word is: {}", word);

    s.clear(); // 这会清空字符串，使其长度为 0

    // println!("the first word is: {}", word); // panic , 因为 word 是一个指向 s 的引用，而 s 已经被清空了
}

fn first_word(s: &String) -> &str {
    &s[..1]
}
