use std::{env, error::Error, fs};

pub struct CmdArgs {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl CmdArgs {
    pub fn build(args: &[String]) -> Result<CmdArgs, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(CmdArgs {
            query,
            file_path,
            ignore_case,
        })
    }
}
pub fn run(args: CmdArgs) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&args.file_path)?;
    // println!("Query: {query}");
    if args.ignore_case {
        for line in search_case_insensitive(&args.query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search(&args.query, &contents) {
            println!("{line}");
        }
    }
    Ok(())
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = vec![];
    for lines in content.lines() {
        if lines.contains(query) {
            result.push(lines);
        }
    }
    return result;
}

pub fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = vec![];
    for lines in content.lines() {
        if lines.to_lowercase().contains(&query.to_lowercase()) {
            result.push(lines);
        }
    }
    return result;
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
        let query = "rUsT";
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
}
