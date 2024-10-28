搜索命令示例：

```
cargo run -- to poem.txt
```

设置大小写不敏感的环境变量：
```
IGNORE_CASE=1 cargo run -- to poem.txt
```

取消环境变量：
```
unset IGNORE_CASE
```