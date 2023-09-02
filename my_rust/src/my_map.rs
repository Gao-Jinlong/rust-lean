// 注意 HashMap 必须先引入，在三个常用集合中 HashMap 最不常用，所以没有被 prelude 自动引用，注意与 struct 区分
use std::collections::HashMap;

pub fn main() {
    // 创建一个空的 HashMap 并使用 insert 增加元素
    let mut scores = HashMap::new();

    // 这里所有的 key 都是 String 所有的值都是 i32
    // 同 vector 一样 HashMap 是同质的，所有键必须是相同类型，值也必须都是相同类型
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // 使用 get 方法获取 map 中的值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // get 方法返回 Option<&V>，如果 HashMap 中没有对应值，get 会返回 None
    // 通过 copied 方法来获取一个 Option<i32> 而不是 Option<&i32> 接着调用 unwrap_or 在 sources 中没该键所对应的项时将其设为 0

    println!("get value from HashMap {}", score);

    // 遍历 HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 这会以任意顺序打印出每个键值对（每次运行顺序都可能不一样）
    for (key, value) in &scores {
        println!("{key} : {value}")
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name 和 field_value 将不在有效
    // println!("HaspMap will get the ownership of value {field_name} {field_value}");

    scores.insert(String::from("Blue"), 25); // 会替换原来的值
    println!("scores {:?}", scores);

    // 使用 entry 添加元素，查看键是否关联了一个值，如果没有就添加
    // entry 方法返回一个 Entry 枚举类型，or_insert 方法在键对应的值存在时返回这个值的可变引用，不存在则将参数作为新值插入并返回新值的可变引用
    scores.entry(String::from("Yellow")).or_insert(60);
    scores.entry(String::from("Green")).or_insert(60);

    println!("{:?}", scores);

    // 文本计数，根据旧的值更新新的值
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    find_middle_and_mode(vec![7, 7, 7, 7, 7, 5, 7, 6, 4, 4, 4, 2, 1, 3, 3]);
    transform_pig_latin("apple");
    transform_pig_latin("rust");

    let mut departments = DepartMents {
        department: HashMap::new(),
    };

    departments.add_member(String::from("Engineering"), String::from("Sally"));
    departments.add_member(String::from("Engineering"), String::from("Amir"));
    departments.add_member(String::from("Sales"), String::from("Bob"));
    departments.add_member(String::from("Sales"), String::from("John"));
    departments.get_members(String::from("Engineering"));
    departments.get_all_members_sorted();
}

// 给定一系列数字，使用 vector 并返回这个列表的中位数和众数。
fn find_middle_and_mode(numbers: Vec<i32>) {
    let mut map = HashMap::new();

    for number in &numbers {
        let count = map.entry(*number).or_insert(0);
        *count += 1;
    }

    let mut middle = 0;
    let mut mode = 0;
    let mut max_count = 0;

    for (number, count) in &map {
        if *count > max_count {
            max_count = *count;
            mode = *number;
        }
    }
    // 排序 numbers
    let mut numbers = numbers.clone();
    numbers.sort();
    // 计算中位数
    let length = numbers.len();
    if length % 2 == 0 {
        middle = (numbers[length / 2] + numbers[length / 2 - 1]) / 2;
    } else {
        middle = numbers[length / 2];
    }

    println!(
        "middle is {middle}, mode is {mode}",
        middle = middle,
        mode = mode
    );
}

// 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！
fn transform_pig_latin(s: &str) {
    let mut map = HashMap::new();
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    for vowel in &vowels {
        map.insert(vowel, true);
    }
    let mut result = String::new();
    if let Some(first) = s.chars().next() {
        if map.contains_key(&first) {
            result = format!("{}-hay", s);
        } else {
            result = format!("{}-{}ay", &s[1..], first)
        }
    }

    println!("the result of transform_pig_latin {} is {}", s, result);
}

// 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。
struct DepartMents {
    department: HashMap<String, Vec<String>>,
}
impl DepartMents {
    fn add_member(&mut self, dpt: String, name: String) {
        let department = self.department.entry(dpt).or_insert(Vec::new());
        department.push(name);

        println!("{:?}", self.department);
    }

    fn get_members(&self, dpt: String) {
        let names = self.department.get(&dpt);
        println!("members of {} is {:?}", dpt, names)
    }

    fn get_all_members_sorted(&self) {
        let mut names = self.department.values().flatten().collect::<Vec<&String>>();
        names.sort();
        println!("all members sorted is {:?}", names)
    }
}
