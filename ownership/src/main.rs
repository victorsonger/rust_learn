fn print_str(s: &str) {
    println!("{}", s);
}

fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    println!("slice == &[2, 3]?: {:?}", slice == &[2, 3]);

    // 比较 slice1 和 slice2 是否指向相同的内存地址
    let are_same_address = std::ptr::eq(slice, &[2, 3]);
    println!(
        "slice and slice2 point to the same address: {}",
        are_same_address
    );
}
