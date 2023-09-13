use adder;

mod common;

// tests 文件夹中的任何文件都被视为测试文件，而不是源文件，所以不会被添加到 crate 的可执行文件中。
// 这意味着 tests 文件夹中的文件不能像 src 文件夹中的文件那样访问 crate 中的私有项。
// 例如，如果在 tests/integration_test.rs 中尝试使用 Rectangle 结构体，就会得到一个错误，因为 Rectangle 结构体是 adder crate 的私有项。
// 为了解决这个问题，需要将 tests/integration_test.rs 中的 use 语句改为 use adder;，这样就可以使用 adder crate 中的任何内容了。
// 这是因为 tests 文件夹中的每个文件都被编译为单独的 crate。
// 这意味着每个文件都会有自己的 crate 根路径，而不是像 src/main.rs 那样有一个隐式的根模块。

// tests 是一个特殊文件夹，cargo test 会自动运行该文件夹下的所有测试文件。
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

// 如果是二进制 crate 并且只包含 src/main.rs 而没有 src/lib.rs，
// 这样就不可能在 tests 目录中创建集成测试并使用 extern crate 导入 src/main.rs 中定义的函数。
// 只有库 crate 才会向其他 crate 暴露了可供调用和使用的函数
// 通常使用一个简单地 main.rs 文件来调用 src/lib.rs 中定义的逻辑，这样可以集成测试 src/lib.rs 中的代码。
