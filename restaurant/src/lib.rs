pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
    mod serving{
        fn take_order(){}

        fn serve_order(){}

        fn take_payment(){}
    }
}

pub fn eat_at_restaurant(){
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }

    fn cook_order(){}

    pub struct Breakfast { // 注意这个结构体具有私有字段，因此需要提供一个公共的关联函数来构造
        pub toast: String,
        seasonal_fruit: String,
    }

    // 如果没有这个公共的关联函数来构造 Breakfast 实例，那么外部代码就不能构造 Breakfast 实例
    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 枚举只需要在 enum 前加 pub 它的所有成员就可以变为公共的
    // 枚举成员默认公有，所以只需要在父级加 pub 就可以使整个枚举变为公共的
    pub enum Appetizer{
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant(){
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please". meal.toast);

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");
}

pub fn eat_at_restaurant(){
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
