fn main() {
    // the code below will cause an error
    // let x = 5;
    // println!("The value of x is: {x}");
    // let x = 6;

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // the const must have a type annotation
    // and can't use mut
    // and can't use the value that can only be obtained in runtime
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}")
}
