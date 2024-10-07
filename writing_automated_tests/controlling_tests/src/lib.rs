fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

/// # Examples
/// ```
/// // 由于 Rust 文档测试在一个独立的测试环境中执行，因此不会自动引入当前模块的内容。
/// // 所以我们不能直接use super::*;
/// // 而是要指定具体的模块名称
/// use controlling_tests::add_two;
/// let sum = add_two(2);
/// assert_eq!(sum, 4);
/// ```
pub fn add_two(a: usize) -> usize {
    a + 2
}

// rust允许测试私有函数
fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        // 默认情况下，通过的测试，其内部的打印会被吞掉，无法看到
        // 如果想要看到，可以使用cargo test -- --show-output
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(value, 5);
    // }

    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        let result = add_two(1000);
        assert_eq!(result, 1002);
    }

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
