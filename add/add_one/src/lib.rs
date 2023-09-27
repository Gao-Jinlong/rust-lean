use rand;

pub fn add_one(x: usize) -> usize {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo test -p add_one 指定测试某个包
    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
