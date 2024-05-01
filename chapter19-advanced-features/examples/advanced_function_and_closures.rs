fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string()) // 闭包
        .collect();

    println!("{:?}", list_of_strings);

    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect(); // 使用函数指针

    println!("{:?}", list_of_strings);
    
    // 每一个我们定义的枚举成员的名字都是一个构建枚举的实例的函数，这意味着我们可以使用这些构造函数作为实现了闭包 trait 的函数指针
    enum Status {
      Value(32),
      Stop,
    }

    let list_of_statuses:Vec<Status> = (0u32..20).map(Status::Value).collect();

    // 返回闭包
    // 闭包表现为 trait (Fn, FnMut, FnOnce) ，这意味着不能直接返回闭包
    // 可以使用实现了期望返回的 trait 的具体类型来替代函数的返回值。但这不能用于闭包，因为它们没有具体类型
    // 例如：不允许使用函数指针 fn 作为返回值，因为它们不是 Sized 的，rust 无法确定它们的大小
    // 因此可以使用 trait 对象
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
