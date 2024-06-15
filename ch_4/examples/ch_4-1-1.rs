#[derive(Debug)]
struct Foo;
impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }

    fn share(&self) {
        println!("\nshared");
    }
}
fn main() {
    let mut foo: Foo = Foo;
    // 根据 rust 的生命周期规则 mutate_and_share 参数 &mut self 和返回值 &self 的生命周期是相同的
    // 因此 foo.mutate_and_share() 虽然返回的是不可变引用，但是参数 &mut self 的生命周期依然存在，所以下一行中的 foo.share() 获取一个不可变引用会导致 panic
    {
        let loan = foo.mutate_and_share();
        // foo.share(); // panic, immutable borrow occurs here
        print!("{:?}", loan);
    }
    foo.share();

    use std::collections::HashMap;
    use std::hash::Hash;

    fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
    where
        K: Clone + Eq + Hash,
        V: Default,
    {
        // match map.get_mut(&key) {
        //     // 这里对 map 的可变借用在读取到 map 中的值后就结束了
        //     // 但是 rust 将这个可变借用的生命周期维持到了下面代码快结束，导致代码块中的可变借用 panic
        //     Some(value) => value,
        //     None => {
        //         map.insert(key.clone(), V::default());
        //         map.get_mut(&key).unwrap()
        //     }
        // }
    }
}
