fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    // 使用生命周期注释语法
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");

    fn longest2(x: &str, y: &str) -> String {
        let result = String::from("really long string");
        result
    }

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result2 = longest2(&string1, string2.as_str());
    println!(" {:?} is the value of {}", result2, "result2");

    // `&string1` 会自动解引用为 `&str`
    let ref_string1 = &string1;
    let as_str_string1 = string1.as_str();

    // 比较这两个值
    if ref_string1 == as_str_string1 {
        println!("ref_string1 and as_str_string1 are the same.");
    } else {
        println!("ref_string1 and as_str_string1 are different.");
    }

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let string_4 = String::from("hello world");
    let first_world_of_4 = first_word(&string_4[..]);
    println!(
        " {:?} is the value of {}",
        first_world_of_4, "first_world_of_4"
    );

    let string_4 = String::from("hello world");
    let arr = &string_4;
    println!(" {:?} is the value of {}", arr, "arr");

    // use std::fmt::Display;

    // fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    // where
    //     T: Display, // 泛型约束 也可以不写这个where，而是在函数名后的尖括号里写： T: Display
    // {
    //     println!("Announcement! {ann}");
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }
}
