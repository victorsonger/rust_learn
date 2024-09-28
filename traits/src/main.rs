// trait
// https://doc.rust-lang.org/book/ch10-02-traits.html

use std::fmt::{Debug, Display};

use traits::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    // Traits as Parameters 作为参数的特征
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    notify(&article);

    // Specifying Multiple Trait Bounds with the + Syntax
    // 使用+语法指定多个特征边界
    pub fn notify_with_display(item: &(impl Summary + Display)) {
        println!("{}", item);
    }

    notify_with_display(&article);

    // Clearer Trait Bounds with where Clauses
    // 使用where来更清晰地划分 Trait 子句的界限
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        // 打印 T 类型的值
        println!("Value of t: {}", t);

        // 打印 U 类型的值（使用 Debug 格式）
        println!("Value of u: {:?}", u);

        // 克隆 T 和 U 类型的值
        let _t_clone = t.clone();
        let _u_clone = u.clone();

        // 可以根据业务逻辑返回一个 i32 值，举例返回 0
        0
    }

    let t = "Hello"; // &str 实现了 Display 和 Clone
    let u = vec![1, 2, 3]; // Vec<i32> 实现了 Debug 和 Clone

    let result = some_function(&t, &u);
    println!("Result: {}", result);
}
