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
        SpreadsheetCell::Float(23.4)
    ];

    for i in &row {
        println!("row: {i:?}");
    }
}
