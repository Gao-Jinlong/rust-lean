struct Struct {
    e: i32,
}

fn main() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    const MAX_POINTS: u32 = 100_000; // 声明常量，使用下划线增加可读性

    println!("MAX_POINTS: {}", MAX_POINTS);

    const _number: isize = 5; // isize 类型的大小取决于计算机架构，32 位架构上是 32 位，64 位架构上是 64 位

    // 整型溢出
    // wrapping_* 按照补码循环溢出规则处理
    assert_eq!(100u8.wrapping_add(1), 101);
    assert_eq!(u8::MAX.wrapping_add(1), 0);
    // checked_* 检查是否溢出，溢出返回 None
    assert_eq!(100u8.checked_add(1), Some(101));
    assert_eq!(u8::MAX.checked_add(1), None);
    // overflowing_* 返回计算结果和是否溢出的标志
    assert_eq!(100u8.overflowing_add(1), (101, false));
    assert_eq!(u8::MAX.overflowing_add(1), (0, true));
    // saturating_* 可以限定计算后的结果不超过目标类型的最大值或低于最小值
    assert_eq!(100u8.saturating_add(1), 101);
    assert_eq!(u8::MAX.saturating_add(1), u8::MAX);

    // 浮点数
    // rust 浮点数的比较运算实现了 std::cmp::PartialEq 而不是和其他数值类型一样实现了 std::cmp::Eq
    // HashMap 数据结构的键必须实现 Eq 所以浮点数不能作为 HashMap 的键
    // assert!(0.1 + 0.2 == 0.3); // error

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc（f32）");
    println!("0.1 + 0.2 = {}", (abc.0 + abc.1).to_bits());
    println!("0.3 = {}", (abc.2).to_bits());

    println!("xyz（f64）");
    println!("0.1 + 0.2 = {}", (xyz.0 + xyz.1).to_bits());
    println!("0.3 = {}", (xyz.2).to_bits());

    assert_eq!(abc.0 + abc.1, abc.2); // success
                                      // assert_eq!(xyz.0 + xyz.1, xyz.2); // panic

    // NaN 不能用来比较
    let x = (-42.0f32).sqrt();
    // assert_eq!(x, x); // panic
    assert_eq!(x.is_nan(), true);

    // 序列（Range）
    let range = 1..5; // [) 左闭右开
    for i in range {
        println!("{}", i);
    }
    for i in 'a'..='z' {
        // [] 包含右边界
        print!("{}", i);
    }
}
