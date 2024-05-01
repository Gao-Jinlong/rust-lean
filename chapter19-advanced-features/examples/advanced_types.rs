// 类型别名
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    println!("takes_long_type");
}
fn returns_long_type() -> Thunk {
    Box::new(|| println!("hi"))
}

// ===========================
// never type ! 在 Rust 中表示永远不会返回的类型
// ! 可以强转为任何类型
fn bar() -> ! {
    println!("bar");
}
let guess = match guess.trim().parse() {
    Ok(_) => 5,
    // Err(_) => "hello", // error, match 分支的返回值类型必须一致
    Err(_) => {
        continue;
    } // continue 也是 ! 类型因此可以和 5 一起返回
};
// 类似的 never 类型还有 panic! 宏

impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

// ==================================
// 动态大小类型 (DST) 和 Sized trait
// 动态大小类型的黄金规则：必须将动态大小类型的值置于某种指针之后。
// 为了处理 DST ，rust 提供了 Sized trait，来决定一个类型的大小是否在编译时可知。此外 rust 隐式的为每个泛型函数增加了 Sized bound
fn generic<T>(t: T) {
    // do something
}
// 等价于
fn generic<T: Sized>(t: &T) {
    // do something
}
// ?Sized 意味着 T 可能是也可能不是 Sized 同时这个注解会覆盖泛型类型必须在编译时拥有固定大小的默认规则。
// 这种意义的 ?Trait 语法只能用于 Sized，而不能用于其他 trait
// 此外将 t 参数的类型从 T 改为 &T 也是因为 T 可能不是 Sized 的，所以需要将其置于某种指针之后。

fn main() {}
