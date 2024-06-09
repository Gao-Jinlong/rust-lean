fn main() {
    #[derive(Debug)]
    enum MyEnum {
        Foo,
        Bar,
    }

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];

    // let foos = v.iter().filter(|x| x == MyEnum::Foo); // panic，因为 x 是一个引用，而 MyEnum::Foo 是一个值
    let foo = v.iter().filter(|x| match x {
        MyEnum::Foo => true,
        _ => false, // _ 代表所有其他情况，_ 通配符表示忽略值
    });
    print!("{:?}\n", foo.collect::<Vec<_>>());

    // 简写
    let foo = v.iter().filter(|x| matches!(x, MyEnum::Foo));

    print!("{:?}\n", foo.collect::<Vec<_>>());

    let foo = 'f';
    assert!(matches!(foo, 'a'..='z'));

    // 变量遮蔽
    let age = Some(30);

    println!("age: {:?}", age); // age: Some(30)

    if let Some(age) = age {
        print!("age: {}\n", age); // age: 30，此处的 age 是等号左侧 Some(age) 中的 age，而不是外部的 age
    };

    print!("age: {:?}\n", age); // age: Some(30)
}
