struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Ginlon"),
        email: String::from("ginlon@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("ginlon@163.com");
    let user2 = build_user(String::from("ginlon"), String::from("123"));
    println!("{}", user2.username);

    // struct update syntax
    let user3 = User {
        email: String::from("some@email.com"),
        ..user1 // 必须放在最后
    };
    println!("{}", user1.sign_in_count);
    println!("{}", user1.active); // 两个值都是 copy trait，所以所有权没有转移，可以访问
    println!("{}", user1.email); // 未使用 user1 的 email，所以所有权没有转移，可以访问
                                 // println!("{}", user1.username); // error, 因为 username 是 drop trait，所有权转移到了 user3

    let back = Color(0, 0, 0);
    let origin = Point(0, 0, 0); // 元组即使看起来一样，也是不同的类型

    // unit-like struct 没有任何字段的类单元结构体, 常用于实现 trait 而不需要存储数据时使用
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        active: true,
        username,
        sign_in_count: 1,
    }
}
