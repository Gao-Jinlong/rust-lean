fn main() {
    // the code below will cause an error
    // let x = 5;
    // println!("The value of x is: {x}");
    // let x = 6;

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // the const must have a type annotation
    // and can't use mut
    // and can't use the value that can only be obtained in runtime
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;

        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // let will rebinding the value to the same name, and the type can be changed, but mut can't
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    let mut spaces = "   ";
    // spaces = spaces.len(); // this will cause an error
    println!("The value of spaces is: {spaces}");
    spaces = "   ";
    println!("The value of spaces is: {spaces}");

    // TUples
    let tup: (i32, f32, u8) = (500, 6.4, 1);
    let (t1, t2, t3) = tup;
    println!("The value of t1 is: {t1}");
    println!("The value of t2 is: {t2}");
    println!("The value of t3 is: {t3}");

    // 不带任何值的元组有个特殊的名称，叫做单元（unit）元组。
    // 这种值以及对应的类型都写作 ()，表示空值或者空的返回类型。
    // 如果表达式不返回任何其他值，则会隐式返回单元值。

    // Array
    let a = [1, 2, 3, 4, 5];
    println!("The value of a[0] is: {}", a[0]);
    let a = [3; 5];
    println!("The value of a[4] is: {}", a[4]);

    // 表达式 vs 语句
    let y = {
        // 这是一个表达式，不是语句，会返回 x + 1 的值
        let x = 3;
        x + 1
    };

    // let y = { // 这是一个语句，不是表达式，默认回一个空的元组即 ()
    //     let x = 3;
    //     x + 1; // 同上面的例子的区别是：这里的分号会使得表达式变成语句，从而返回一个空的元组
    // };

    println!("The value of y is: {y}");

    // function
    another_function(5);
    let x = five();
    println!(
        "The value of five() is: {}, the value of x is {}",
        five(),
        x
    );
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5 // 函数隐式返回最后一个表达式的值
}
