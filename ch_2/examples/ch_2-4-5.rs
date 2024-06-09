fn main() {
    // let s = [String::from("hello"); 3]; // panic，因为 String 没有实现 Copy trait
    let s: [String; 3] = std::array::from_fn(|_i| String::from("hello"));
    print!("{:?}\n", s);

    let one = [1, 2, 3];
    let two: [u8; 3] = [1; 3];

    let arrays: [[u8; 3]; 4] = [one, two, two, one];

    for a in &arrays {
        print!("{:?}\n", a);

        for n in a.iter() {
            print!("{}\n", n);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("{:?} = {}", a, sum);
    }
}
