pub fn main(){
  // 创建一个空的 vector，vector 在一个单独的数据结构中存储多个值，它们在内存中彼此相邻
  let v:Vec<i32> = Vec::new();

  // 使用 vec! 宏创建一个 vector，rust 可以根据初始值推断出 vector 的类型
  let v = vec![1,2,3];

  // 更新 vector
  let mut v = Vec::new();
  v.push(4);
  v.push(5);
  v.push(6); // 依旧可以推导出 vector 的类型

  // 访问 vector 中的元素
  let v = vec![1,2,3,4,5];

  let third: &i32 = &v[2];
  // let over_thread = &v[100]; // panic 使用 [] 访问 vector 时，如果索引超出了 vector 的长度，Rust 会引发一个 panic，终止程序
  println!("The third element is {third}");

  let third: Option<&i32> = v.get(2);
  // let over_thread = v.get(100); // None 使用 get 方法时，如果索引超出了 vector 的长度，它只会返回 None 而不会 panic
  match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
  }

  for i in &v{
    println!("enumerate vector {i}")
  }

  let mut v = vec![100,23,11];
  for i in &mut v{
    *i += 30; // * 解引用运算符，获取 i 的值
  }
  for i in &v{
    println!("enumerate vector {i}")
  }

  // vector 结合 enum 可以存储不同类型的值
  enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];
  
}