use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

// fn main() {
//     // 故意读取一个不存在的文件
//     // let greeting_file_result: Result<File, std::io::Error> = File::open("hello.txt");
//     let greeting_file_result: Result<File, std::io::Error> = File::open("not_exist.txt");

//     let greet_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("not_exist.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {e:?}"),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {other_error:?}");
//             }
//         },
//     };

//     println!(" {:?} is the value of {}", greet_file, "greet_file");

//     // 使用闭包处理同样的逻辑
//     let greeting_file = File::open("new.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("new.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {error:?}");
//             })
//         } else {
//             panic!("Problem opening the file: {error:?}");
//         }
//     });

//     println!(" {:?} is the value of {}", greeting_file, "greeting_file");

//     // 报错的情况下，使用expect来自定义报错消息
//     // File::open("really_error.txt")
//     // .expect("really_error.txt should be included in this project");

//     // 调用传播错误的方法
//     let read_username_result = read_username_from_file_with_operator().unwrap();
//     println!(
//         " {:?} is the value of {}",
//         read_username_result, "read_username_result"
//     );
// }

// 传播错误
fn read_username_from_file() -> Result<String, io::Error> {
    // 可以正常打开并读取文本内容的文件
    let username_file_result = File::open("user_name.txt");

    // 试试注释下面这行，主动触发打开文件的错误
    // let username_file_result = File::open("user_name_false.txt");

    // 再试试注释下面这行，主动触发读取文件的错误
    // let username_file_result = File::open("image.jpeg");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        // 如果打开文件出错，fn read_username_from_file在这里就return了，返回包含io::Error的Err
        // Os { code: 2, kind: NotFound, message: "No such file or directory" }
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        // 进到这里，说明文件可以正常打开
        // 并读取合法的UTF-8字符串
        // username： "张三"
        Ok(_) => Ok(username),
        // 如果读取文件出错，fn read_username_from_file在这里就return了，
        // 返回包含io::Error的Err:  Error { kind: InvalidData, message: "stream did not contain valid UTF-8" }
        // 不过由于是此函数的最后一行，所以不需要加return，它也会是read_username_from_file的返回值
        Err(e) => Err(e),
    }
}

// # 传播错误的一种快捷方式： "?"操作符
fn read_username_from_file_with_operator() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// main函数也可以不返回()类型
// 而是返回Result<(), E>类型
// 比如下面这个
// 当main函数返回Result<(), E>时，如果main返回Ok(())则可执行文件将以0值退出；如果main返回Err值，则可执行文件将以非零值退出
fn main() -> Result<(), Box<dyn Error>> {
    // let greeting_file = File::open("hello.txt")?;
    // 这样引发错误，程序的exit code会是2
    let greeting_file_error = File::open("hello3.txt")?;

    Ok(())
}
