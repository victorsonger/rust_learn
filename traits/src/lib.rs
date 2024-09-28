use std::fmt;

// 定义特征 trait
pub trait Summary {
    // fn summarize(&self) -> String; // 可以只定义方法签名

    // 也可以提供默认实现
    fn summarize(&self) -> String {
        // 默认实现可以调用同一特征中的其他方法，即使这些其他方法没有默认实现
        format!("(Read more from {}...)", self.summarize_author())
    }

    // 没有默认实现的方法
    fn summarize_author(&self) -> String;
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 使用默认实现来汇总NewsArticle的实例，我们指定一个空的impl块 impl Summary for NewsArticle {}
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl fmt::Display for NewsArticle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.headline, self.location)
    }
}
