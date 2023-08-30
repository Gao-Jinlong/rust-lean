struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{ // &self 为 self:&Self 的语法糖，Self 实际上是当前块所属类型的别名
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
    // getter 方法，rust 不会自动实现 getter，常用于私有化字段
    fn width(&self) -> u32{
        self.width
    }
    // associated function 关联函数，不以 self 作为参数，类似于静态方法，常用于构造器，例如 String::from
    fn square(size:u32) -> Self {
        Self{
            width: size,
            height: size,
        }
    }
}

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
    let _user3 = User {
        email: String::from("some@email.com"),
        ..user1 // 必须放在最后
    };
    println!("{}", user1.sign_in_count);
    println!("{}", user1.active); // 两个值都是 copy trait，所以所有权没有转移，可以访问
    println!("{}", user1.email); // 未使用 user1 的 email，所以所有权没有转移，可以访问
                                 // println!("{}", user1.username); // error, 因为 username 是 drop trait，所有权转移到了 user3

    let _back = Color(0, 0, 0);
    let _origin = Point(0, 0, 0); // 元组即使看起来一样，也是不同的类型

    // unit-like struct 没有任何字段的类单元结构体, 常用于实现 trait 而不需要存储数据时使用
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
    

    let rect1 = Rectangle{
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {} square pixels.", area(&rect1));
    println!("rect1 is {:?}", rect1); // debug trait

    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);

    println!("The area of rectangle is {} square pixels. by method", rect1.area());

    // 自动引用和解引用
    // rust 会自动帮我们解引用，这样就可以直接使用方法
    let rect2 = &rect1;
    println!("auto deref: {} (&rect2).width()", (&rect2).width()); 
    println!("auto deref: {} rect2.width()", rect2.width());  // 在 c++ 中，需要使用 -> 运算符访问指针的成员，而 rust 会自动帮我们解引用

   let rect2 = Rectangle {
    width:20,
    height:40,
   };
   let rect3 = Rectangle {
    width:60,
    height:40,
   };

   println!("rect1 can hold rect2:{}", rect1.can_hold(&rect2));
   println!("rect1 can hold rect3:{}", rect1.can_hold(&rect3));

   let square_rect = Rectangle::square(10);
   println!("associate function square_rect: {:?}", square_rect);
}    


fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        active: true,
        username,
        sign_in_count: 1,
    }
}
