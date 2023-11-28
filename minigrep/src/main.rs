use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    println!("query = {}", config.query);
    println!("filename = {}", config.filename);
    println!();

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        // args[0] 为当前执行的二进制文件名称，即 target/debug/minigrep.exe
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}