// https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html
fn main() {
    // println!("Hello, world!");
    // panic!("Crash and burn");

    let v = vec![1, 2, 3];

    // 这里的报错默认只会回溯到这一行
    // 如果想要查看整个回溯栈
    // 则需要在执行的时候设置环境变量
    // RUST_BACKTRACE=1 cargo run
    v[99];
}
