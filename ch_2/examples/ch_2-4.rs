#![allow(unused_variables)]
type File = String;

fn open(f: &mut File) -> bool {
    true
}
fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    // ! 表示这个函数不会返回，即发散函数
    unimplemented!(); // 未实现的函数
    todo!(); // 未完成的函数
}

fn main() {
    let mut f1 = File::from("test.txt");
    open(&mut f1);
    // read(&mut f1, &mut vec![]);
    close(&mut f1);
}
