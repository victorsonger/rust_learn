// cargo test -- --nocapture

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// shoes_in_size函数将鞋子向量的所有权和鞋子尺寸作为参数。它返回一个仅包含指定尺寸鞋子的向量
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // 调用into_iter来创建一个拥有向量所有权的迭代器。然后我们调用filter将该迭代器调整为一个新的迭代器，该迭代器仅包含闭包返回true元素
    shoes.into_iter().filter(|s: &Shoe| s.size == shoe_size).collect()
}

#[test]
fn iterator_demonstration() {
    println!(
        "\n#  示例 13-12：在迭代器上调用next方法  \n---------------------------------------------"
    );
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
