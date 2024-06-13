use std::collections::HashMap;

fn main() {
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    // collect 会根据类型自动尝试转换
    let teams_map: HashMap<_, _> = teams_list.into_iter().collect();
    println!("{:?}", teams_map);

    let teams_map: Vec<_> = teams_map.into_iter().collect();
    println!("{:?}", teams_map);

    // 更新 HashMap 中的值
    let mut scores = HashMap::new();

    scores.insert("Blue", 10);

    // 覆盖旧值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));

    // 查询新插入值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    // 查询 Yellow 值，若不存在啧插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5);

    // 若存在则不插入
    let v = scores.entry("Yellow").or_insert(10);
    assert_eq!(*v, 5);

    // 统计单词出现次数
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // HashMap 的 key 必须实现 std::cmp::Eq 因此 f32 和 f64 不能作为 key
}
