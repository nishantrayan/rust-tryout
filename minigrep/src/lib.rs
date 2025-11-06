use colored::*;
use std::env;
use std::error::Error;
use std::fs;
struct Highlight<'a> {
    line: &'a str,
    highlight_ranges: Vec<(usize, usize)>,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for highlight in results {
        for (index, chr) in highlight.line.chars().enumerate() {
            let is_highlighted = highlight
                .highlight_ranges
                .iter()
                .any(|(start, end)| (*start..*end).contains(&index));
         
            if is_highlighted {
                print!("{}", chr.to_string().green());
            } else {
                print!("{}", chr);
            }
        }
        println!();
    }
    Ok(())
}

fn highlight_line<'a>(line: &'a str, query: &str) -> Highlight<'a> {
    let mut highlight_ranges = Vec::new();
    for i in query.len() - 1..line.len() {
        let start = i + 1 - query.len();
        let end = i + 1;
        let token = &line[start..end];
        if token.to_lowercase() == query.to_lowercase() {
            highlight_ranges.push((start, end));
        }
    }
    Highlight {
        line,
        highlight_ranges,
    }
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<Highlight<'a>> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .map(|line| highlight_line(line, query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<Highlight<'a>> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .map(|line| highlight_line(line, query))
        .collect()
}
pub struct Config {
    pub query: String,
    pub file: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            file,
            case_sensitive,
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn one_result() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";
//         let results = search(query, contents);
//         assert_eq!(results, vec!["safe, fast, productive."]);
//     }

//     #[test]
//     fn case_sensitive() {
//         let query = "rUst";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Trust me.";
//         let results = search_case_insensitive(query, contents);
//         assert_eq!(results, vec!["Rust:", "Trust me."]);
//     }
// }
