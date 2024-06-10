fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let word_list = vec![1, 2];
    let result = add(word_list[0], word_list[1]);
    println!("The concatenated word is {}", result);

    let position = Point { x: 5, y: 10 };
    print!(
        "position.x = {}, position.y = {}\nposition.area = {}\n",
        position.x(),
        position.y(),
        position.area()
    );

    let arr = [1, 2, 3];
    display_array(&arr);
    display_array2(arr);
}

fn largest<T>(list: &[T]) -> T
where
    T: std::cmp::PartialOrd + Copy, // where 从句指定泛型 T 必须实现 PartialOrd 和 Copy trait
{
    let mut largest = list[0];

    // T 实现了 Copy trait，所以可以使用 &item 复制 item 的值
    for &item in list.iter() {
        // T 实现了 PartialOrd trait，所以可以使用 > 比较 item 和 largest
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

// 结构体中的泛型
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}
// 针对特定类型的 Point 实现方法
impl Point<i32> {
    fn area(&self) -> i32 {
        self.x * self.y // 需要实现 Mul trait 和 Copy trait
    }
}

// const 泛型
// 此处必须使用引用才能够适配不同长度的数组
fn display_array<T>(arr: &[T])
where
    T: std::fmt::Debug,
{
    print!("{:?}", arr)
}
// 当泛型参数是常量时，可以使用 const 声明泛型参数，如下：
fn display_array2<T, const N: usize>(arr: [T; N])
where
    T: std::fmt::Debug,
{
    print!("{:?}", arr)
}
