# Rust-Toolbox
使用 Rust 编写的小工具


## 1. minigrep
项目功能：
- 根据给定的查询字符串，到指定的文件中查找包含该查询字符串的行，最后打印出来

相关操作：
```bash
# 克隆项目
$ git clone https://github.com/wtraceback/Rust-Toolbox.git

# 切换至目录
$ cd Rust-Toolbox
$ cd minigrep

# 正常运行（查找 poem.txt 文件下包含 to 字符串的行，输出到终端，区分查询字符串的大小写）
$ cargo run to poem.txt

# 环境变量相关
# 设置环境变量
# $ export CASE_INSENSITIVE=1
# 查看环境变量
# $ echo $CASE_INSENSITIVE
# 移除设置的环境变量
# $ unset CASE_INSENSITIVE

# 使用环境变量的情况下运行（不区分大小写）
$ CASE_INSENSITIVE=1 cargo run to poem.txt

# 测试
$ cargo test

# 标准错误 和 标准输出
# 标准输出，输出信息重定向到文件中
$ cargo run to poem.txt > output.txt
# 标准错误，错误信息直接在命令行展示（由于没有给出要查询的字符串和文件名，会报错）
$ cargo run

```
