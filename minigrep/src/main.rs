use std::env;       // 使用 Rust 标准库提供的 std::env::args 函数获取传递给 minigrep 的命令行参数迭代器

fn main() {
    // 使用 collect 函数将迭代器转换成一个包含所有迭代器产出值的动态数组
    // 运行：$ cargo run test test.name
    // 输出：["target\\debug\\minigrep.exe", "test", "test.name"]
    let args: Vec<String> = env::args().collect();

    // args[0] 为当前执行的二进制文件名称，即 target/debug/minigrep.exe
    let query = &args[1];
    let filename = &args[2];

    println!("query = {}", query);
    println!("filename = {}", filename);
}
