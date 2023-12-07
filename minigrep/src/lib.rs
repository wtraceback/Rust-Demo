use std::fs;
use std::error::Error;


pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // args[0] 为当前执行的二进制文件名称，即 target/debug/minigrep.exe
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("query = {}", config.query);
    println!("filename = {}", config.filename);
    println!();

    let contents = fs::read_to_string(config.filename)?;

    search(&config.query, &contents);

    Ok(())
}

fn search(query: &str, content: &str) {
    for line in content.lines() {
        if line.contains(query) {
            println!("{}", line);
        }
    }
}