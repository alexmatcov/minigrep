use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = determine_case_sensitivity(args);

        Ok(Config { 
            query,
            file_path,
            ignore_case, 
        })
    }
}

// the defined return type int the signature for the error
// implements the Error trait and allows returning different
// types of error (`dyn` means dynamic).
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    
    for line in contents.lines() {
        // converting the `query` to lowercase creates a new String data type
        // instead of continuing to store a string literal, thus reference is called
        if  line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn determine_case_sensitivity(args: &[String]) -> bool {
    // check for command line argument first (higher precedence)
    for arg in args.iter().skip(3) {
        match arg.as_str() {
            "-i" => return true,
            "-s" => return false,
            _ => continue,
        }
    }

    // IGNORE_CASE environment variable is passed to the fn env:var
    // which returns a Result that is converted to bool by is_ok().
    // We don't care about `unwrap` or `expect`, we just need to check
    // if the env var is set or not.
    env::var("IGNORE_CASE").is_ok()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn control_sensitivity() {
        let args: Vec<String> = vec![
            String::from("taregt/debug/minigrep"),
            String::from("to"),
            String::from("poem.txt"),
            String::from("-s")];
        let ignore_case = true;

        assert_ne!(ignore_case, determine_case_sensitivity(&args))
    }
}