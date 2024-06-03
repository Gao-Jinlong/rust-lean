fn main() {
    let penguin_data = "\
   common name,length (cm)
   Little penguin,33
   Yellow-eyed penguin,65
   Fiordland penguin,60
   Invalid,data
   ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        // enumerate 将迭代器转换为 (index, value) 元组
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // <_> 表示 Vec 中的元素类型由编译器推断
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        // cfg! 宏用于检查是否启用了某个特性
        // debug_assertions 是一个编译器内置的特性，用于检查是否启用了调试模式
        // 如果启用了调试模式，则输出调试信息
        // 使用命令 cargo run --release 可以关闭调试模式
        if cfg!(debug_assertions) {
            // eprintln! 输出到标准错误流 println! 输出到标准输出流
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];

        // ::<TYPE> ‘turbofish’ 语法，用于帮助编译器推断类型
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }

    let _a = 10; // 默认是 i32 类型
    let _b: i32 = 20; // 显式指定类型
    let _c = 30i32; // 使用后缀指定类型
    let _d = 30_i32; // 使用下划线分隔数字
}
