pub fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    // 迭代器都是惰性的，因此只有调用一个消费（consume）迭代器的方法时，才会真正地执行。
    // Iterator trait 中定义了另一类方法迭代适配器（iterator adaptors），允许我们将当前的迭代器变为不同类型的迭代器，如 map() 方法。
    // 这里调用 collect() 方法，将迭代器转换为一个 Vec<i32> 类型的值。
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("\nv2 is {:?}", v2);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                }
            ]
        );
    }
}
