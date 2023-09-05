pub fn main() {
    let number_list: Vec<i32> = vec![34, 20, 40, 50, 100, 60];
    let result: &i32 = largest(&number_list);

    println!("The largest number is {}.", result);

    let integer_and_float = Point { x: 4, y: 5.0 };
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// 编译器会通过单态化将泛型代码转换为具体类型代码
// 这样做的好处是泛型代码不会影响运行时性能

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generic struct
struct Point<T, U> {
    x: T,
    y: U,
}
// generic method
// 此处可以使用与结构体定义时不同的泛型名称
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}
// generic method with concrete type
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> Point<X1, Y1> {
    // 方法中使用不同泛型
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// generic enum
enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
