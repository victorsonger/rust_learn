use std::{thread, time::Duration};

// 13.1 https://doc.rust-lang.org/book/ch13-01-closures.html
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    println!("\n#  场景如下：我们的 T 恤公司经常向我们邮件列表中的某人赠送一件独家限量版衬衫作为促销。邮件列表中的人们可以选择将他们喜欢的颜色添加到他们的个人资料中。如果被选为免费衬衫的人有他们最喜欢的颜色套装，他们就会得到该颜色的衬衫。如果此人没有指定最喜欢的颜色，他们会选择公司目前最常用的颜色   \n---------------------------------------------");
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // println!("\n#  一个给闭包注释的例子  \n---------------------------------------------");
    // let res = demo_closure_annotation();
    // println!(" {:?} is the value of {}", res, "res");

    println!("\n#  示例 13-4：定义和调用捕获不可变引用的闭包  \n---------------------------------------------");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    println!("\n#  示例 13-5定义和调用捕获可变引用的闭包   \n---------------------------------------------");
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let mut borrows_mutably = || list.push(7);
    // 加上这句打印就会报错了
    // 当定义borrows_mutably时，它捕获对list可变引用。调用闭包后我们不再使用闭包，因此可变借用结束。在闭包定义和闭包调用之间，不允许进行不可变的借用打印，因为当存在可变借用时不允许其他借用。尝试添加一个println!在那里查看您收到的错误消息！
    // println!("Before calling closure: {list:?}");
    borrows_mutably();
    println!("After calling closure: {list:?}");

    println!("\n# 13-6 move 强制闭包获取所有权  \n---------------------------------------------");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // 在新线程而不是主线程中打印向量
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
    // 这里再想去拿list就会报错了，因为上面的闭包已经通过move强制获取了所有权
    // println!("After calling closure: {list:?}");

}

// 一个给闭包注释的例子
fn demo_closure_annotation() -> u32 {
    // 闭包也可以注释
    // 还可以复制给变量
    // 只是会有些冗余
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    expensive_closure(23)

    // fn add_one_v1(x: u32) -> u32 {
    //     x + 1
    // }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = { |x| x + 1 };
    // let add_one_v4 = |x| x + 1;
}
