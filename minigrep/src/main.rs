// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html
use std::env;
use std::process;

use minigrep::Config;

/**
保留在main函数中的职责应限于以下内容：
Calling the command line parsing logic with the argument values
使用参数值调用命令行解析逻辑
Setting up any other configuration
设置任何其他配置
Calling a run function in lib.rs
调用lib.rs中的run函数
Handling the error if run returns an error
如果run返回错误，则处理错误
 */
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        // 将错误信息打印到标准错位流
        // 这样它就不会出现在由 > 重定向到output.txt文件中
        // 而只会出现在终端
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config.clone()) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    // 如果想多次使用config调用run，就需要对Config实现Clone，并在前面的调用中使用config.clone()
    // if let Ok(res) = minigrep::run(config) {
    //     println!("Run was successful, result: {:?}", res);
    //     process::exit(1);
    // }
}
