fn main() {
    let bar = Bar;
    bar.foo();

    // 当 trait 中至少一个方法包含编译器无法验证的不变式（invariant）时 trait 是不安全的
    // 可以在 trait 前加上 unsafe 关键字将 trait 声明为不安全的，实现这个 trait 时也需要加上 unsafe 关键字

    // 编译器会自动为完全由 Sync 和 Send 类型组成的类型自动实现它们，如果实现了一个包含一些不是 Sync 或 Send 的类型，比如裸指针，并希望将此类型标记为 Send 或 Sync，则必须使用 unsafe
    // rust 不能验证这些类型是否满足 Send 和 Sync 的安全要求，因此需要程序员来保证
}

unsafe trait Foo {
    fn foo(&self);
}
unsafe impl Foo for i32 {
    fn foo(&self) {
        println!("foo");
    }
}

struct Bar;

unsafe impl Foo for Bar {
    fn foo(&self) {
        println!("foo");
    }
}
