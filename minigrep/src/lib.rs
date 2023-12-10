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
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "test";
        let s = "
111
222 test ??
333
        ";

        assert_eq!(
            vec!["222 test ??"],
            search(query, &s)
        )
    }
}