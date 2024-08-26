# 常用方法和技巧记录

## 用于打印变量类型的通用方法

```
use std::any::type_name;

// 打印变量类型
fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}
```