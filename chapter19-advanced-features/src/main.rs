use std::slice;

fn main() {
    // raw pointers 裸指针
    let mut num = 5;

    // 可以在非unsafe代码中创建不可变和可变的裸指针，但是不能在非unsafe代码中解引用裸指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 创建指向任意内存地址的裸指针
    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("!r2 is: {}", *r2);
    }

    // 不安全函数
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    // 创建不安全代码的安全抽象
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..]; // 可变切片

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // 使用 extern 函数调用外部代码
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// fn split_at_mut(values:&mut [i32], mid:usize) -> (&mut [i32], &mut [i32]) {
//     let len = values.len();

//     assert!(mid <= len);

//     (&mut values[..mid], &mut values[mid..]) // rust 不能理解我们要借用这个 slice 的两个不同的部分，它只知道我们借用了同一个 slice 两次，因此会报错
// }

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr(); // 获取 slice 的裸指针

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid), // 使用 from_raw_parts_mut 函数创建一个新的 slice，这个函数是 unsafe 的，因为它接受一个裸指针作为参数
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// 使用 extern 函数调用外部代码
extern "C" {
    // 定义了外部函数所使用的 ABI（application binary interface  应用程序二进制接口），ABI 定义了如何在汇编语言层面调用次函数。 "C" ABI 是最常见的，并遵循 C 语言的 ABI
    fn abs(input: i32) -> i32;
}

// 从其他语言调用 Rust 函数
#[no_mangle] // 禁用 Rust 编译器的名称重整（name mangling）功能，mangling 是编译器增加到函数名中的一些额外信息
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
