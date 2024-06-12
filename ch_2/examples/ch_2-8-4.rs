// /// ------------------------
// ///  关联类型（associated types）
// /// ------------------------
// /// 在 trait 定义中使用关联类型，可以定义一个占位符类型，用于在实现 trait 时指定具体类型。
// pub trait Iterator {
//     type Item; // 在 trait 中定义一个关联类型，直到实现 trait 时才需要指定具体类型

//     fn next(&mut self) -> Option<Self::Item>;
// }

// struct Counter {
//     count: u32,
//     limit: u32,
// }
// impl Iterator for Counter {
//     type Item = u32; // 实现 trait 时指定具体类型
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.count < self.limit {
//             self.count += 1;
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }
// /// panic 关联类型定义的 trait 只能实现一次
// // impl Iterator for Counter {
// //     type Item = String; // 实现 trait 时指定具体类型
// //     fn next(&mut self) -> Option<Self::Item> {
// //         if self.count < self.limit {
// //             self.count += 1;
// //             Some(self.count.to_string())
// //         } else {
// //             None
// //         }
// //     }
// // }

// fn main() {
//     let mut c = Counter {
//         count: 0,
//         limit: 10,
//     };

//     while let Some(item) = c.next() {
//         println!("{}", item);
//     }
// }

/// ------------------------
///  使用泛型
/// ------------------------
struct Counter {
    count: u32,
    limit: u32,
}
pub trait Iterator<Item> {
    fn next(&mut self) -> Option<Item>;
}
impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        if self.count < self.limit {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
impl Iterator<String> for Counter {
    fn next(&mut self) -> Option<String> {
        if self.count < self.limit {
            self.count += 1;
            Some(self.count.to_string())
        } else {
            None
        }
    }
}

fn main() {
    let mut c = Counter {
        count: 0,
        limit: 10,
    };

    // 由于实现了两个不同的 Iterator，所以需要指定具体类型调用
    // while let Some(item) = <Counter as Iterator<u32>>::next(&mut c) {
    //     println!("{}", item);
    // }

    // 完全限定语法，用于指定具体类型，可用于解决多个不同作用域的相同名称的冲突
    // 通常只需要使用 Trait::method 即可，编译器会根据 self 的类型自动选择正确的实现，但是在某些情况下没有 self （如 关联函数）
    // 就需要使用 完全限定语法 <Type as Trait>::method
    while let Some(item) = <Counter as Iterator<String>>::next(&mut c) {
        println!("{}", item);
    }
}
