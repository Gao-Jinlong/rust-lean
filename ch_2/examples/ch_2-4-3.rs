#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("xxx"), // 使用 .. 语法会转移所有权
        active: true,                  // 原始类型实现了 Copy trait，所以这里不会转移所有权
        sign_in_count: 1,
    };

    print!("user1: {:?}\n", user1.username);

    let user2 = User {
        email: String::from("someone"),
        ..user1 // 解构必须放在最后
    };

    // print!("user1.name: {}\n", user1.username); // panic ，因为 user1.username 已经被移动了

    // 没有转移所有权的变量可以继续使用
    print!(
        "user1: {} {} {}\n",
        user1.email, user1.active, user1.sign_in_count
    );
    print!("user2: {:?}", user2);

    // 元组结构体
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    print!("black: {} {} {}\n", black.0, black.1, black.2);

    // 类单元结构体
    // 不关心内容，只关心行为
    struct AlwaysEqual;
    impl AlwaysEqual {
        fn equal(&self) -> bool {
            true
        }
    }

    let a = AlwaysEqual;
    print!("AlwaysEqual: {}\n", a.equal());

    // 结构体中的引用数据必须有生命周期
    #[derive(Debug)]
    struct User2<'a> {
        username: &'a str, // 编译错误，必须指定生命周期
        email: &'a str,
        sign_in_count: u64,
        active: bool,
    }

    let username = String::from("someone");
    let email = String::from("xxx@xxx.com");
    let user = User2 {
        username: &username,
        email: &email,
        sign_in_count: 1,
        active: true,
    };

    print!("user: {:#?}\n", user);
    // {:?} 和 {:#?} 使用 Debug trait 输出 {} 使用 Display trait 输出

    dbg!()
}
