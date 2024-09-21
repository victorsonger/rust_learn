struct Example {
    number: i32,
}

impl Example {
    fn boo() {
        println!("boo! Example::boo() was called!");
    }

    fn answer(&mut self) {
        self.number += 42;
    }

    fn get_number(&self) -> i32 {
        self.number
    }
}




fn main() {
    println!("Hello, world!");
    // 实例化 Example 结构体
    let mut example = Example { number: 0 };

    Example::boo();

    example.answer();

     // 调用方法 get_number 并打印结果
     let number = example.get_number();
     println!("The number is {}", number);
    
}
