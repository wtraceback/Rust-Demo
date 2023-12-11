use std::fs;
use std::error::Error;
use std::env;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // args[0] 为当前执行的二进制文件名称，即 target/debug/minigrep.exe
        let query = args[1].clone();
        let filename = args[2].clone();
        // 只要设置了环境变量，那么结果就为 true，无论 CASE_INSENSITIVE 被设置成什么值
        let case_insensitive = env::var("CASE_INSENSITIVE").is_ok();

        Ok(Config { query, filename, case_insensitive })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // 不区分大小写

    let query: String = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "test";
        let s = "
111
222 test ??
333
Test
        ";

        assert_eq!(
            vec!["222 test ??"],
            search(query, &s)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "test";
        let s = "
111
222 test ??
333
Test
        ";

        assert_eq!(
            vec!["222 test ??", "Test"],
            search_case_insensitive(query, &s)
        )
    }
}