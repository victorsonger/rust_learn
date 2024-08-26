use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // println!("score is {:?}", scores);

    let team_name = String::from("Blue");
    let ss = &team_name;
    let sss = "Blue";
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // 由于Rust 使用deref 强制转换，所以
    // 下面一行得到的结果和上面是一样的
    // let score = scores.get("Blue").copied().unwrap_or(0);

    println!("score is {score}");

    // 迭代
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // 所有权

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("{}", field_name);

    // 没有值的时候再插入 有值的时候不动
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!(" {:?} is the value of {}",  scores, "scores");

    // 统计单词的出现次数

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!(" {:?} is the value of {}",  map, "map");
    
}
