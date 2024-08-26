fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = v.get(0);
    match first {
        Some(first) => println!("The first element is: {first}"),
        None => println!("There is no first element."),
    }

    // v.push(6);

    // println!("The first element is {:#?}", first);

    // 迭代
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(23.4),
    ];

    for i in &row {
        println!("row: {i:?}");
    }

    // 所有权
    let mut own_vec = Vec::new();
    own_vec.push(String::from("Blue"));
    println!("{}: {:?}", "own_vec", own_vec);
    let item_red = String::from("Red");
    own_vec.push(item_red);

    // 下面一行会编译报错
    // （
    // borrow of moved value: `item_red`
    // value borrowed here after move
    // ）
    // println!("{}: {:?}", "item_red", item_red);
}
