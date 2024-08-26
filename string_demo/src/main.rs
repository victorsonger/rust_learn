// https://doc.rust-lang.org/book/ch08-02-strings.html

use std::any::type_name;

// 打印变量类型
fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}
fn main() {
    let hello = "Здравствуйте";

    let s = &hello[2..4];

    println!("s is {s}");

    // 迭代字符
    for c in hello.chars() {
        println!("c is {c}");
    }
    // 迭代byte
    // 迭代字符
    for b in hello.bytes() {
        println!("b is {b}");
    }
}
