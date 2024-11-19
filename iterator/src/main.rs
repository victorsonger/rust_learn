// 13.2 https://doc.rust-lang.org/book/ch13-02-iterators.html
fn main() {
    println!(
        "\n#  示例 13-11：在for循环中使用迭代器  \n---------------------------------------------"
    );
    let v1 = vec![1, 2, 3];
    println!("在 Rust 中，迭代器是惰性的");
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    println!("\n#  证明在 Rust 中两个 &1 指向的内存地址不相等 （但是由于rust的优化，导致实际场景中会相等）\n---------------------------------------------");
    let a = create_reference(true); // &1
    let b = create_reference(false); // &2
    let c = create_reference(true); // &1

    // 将引用转换为裸指针
    let a_ptr = a as *const i32;
    let b_ptr = b as *const i32;
    let c_ptr = c as *const i32;

    // 打印指针地址
    println!("Address of a: {:p}", a_ptr);
    println!("Address of b: {:p}", b_ptr);
    println!("Address of c: {:p}", c_ptr);

    // 比较指针地址
    if a_ptr == b_ptr {
        println!("a and b point to the same memory location.");
    } else {
        println!("a and b point to different memory locations.");
    }

    // 比较指针地址
    if a_ptr == c_ptr {
        println!("a and c point to the same memory location.");
    } else {
        println!("a and c point to different memory locations.");
    }

    println!("\n#  调用迭代器适配器map来创建一个新的迭代器  \n---------------------------------------------");
    let v1: Vec<i32> = vec![1, 2, 3];

    // v1.iter().map(|x| x + 1); // 这里必须要使用 否则会警告 因为map的结果是一个迭代器，而迭代器是惰性的，如果不用，就没有任何价值了

    println!("\n#  13-15 收集了对map到向量的调用返回的迭代器进行迭代的结果。该向量最终将包含原始向量中每一项加 1  \n---------------------------------------------");
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

fn create_reference(isone: bool) -> &'static i32 {
    if isone {
        return &1;
    }
    &2
}
