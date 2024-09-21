// use std::iter;

// https://doc.rust-lang.org/book/ch10-01-syntax.html
// 在函数中使用泛型
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 在结构体Struct中使用泛型
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointMuti<X1, Y1> {
    x: X1,
    y: Y1,
}

// 结构定义中的通用类型参数并不总是与在同一结构的方法签名中使用的参数相同
impl<X1, Y1> PointMuti<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointMuti<X2, Y2>) -> PointMuti<X1, Y2> {
        PointMuti {
            x: self.x,
            y: other.y,
        }
    }
}

#[derive(Debug)]
struct PointMutitype<T, U> {
    x: T,
    y: U,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!(" {:?} is the value of {}", integer.x, "integer.x");
    println!(" {:?} is the value of {}", float.y, "float.y");

    let integer_and_float = PointMutitype { x: 5, y: 4.0 };
    println!(
        " {:?} is the value of {}",
        integer_and_float.x, "integer_and_float.x"
    );
    println!(
        " {:?} is the value of {}",
        integer_and_float.y, "integer_and_float.y"
    );

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p2 = Point {
        x: 3.0f32,
        y: 4.0f32,
    };
    println!(
        " {:?} is the value of {}",
        p2.distance_from_origin(),
        "p2.distance_from_origin()"
    );

    let p1 = PointMuti { x: 5, y: 10.4 };
    let p2 = PointMuti { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
