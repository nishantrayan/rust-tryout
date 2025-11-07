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
        println!("Case sensitive search");
        search(&config.query, &contents)
    } else {
        println!("Case insensitive search");
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

fn highlight_line<'a>(line: &'a str, query: &str, case_sensitive: bool) -> Highlight<'a> {
    let mut highlight_ranges = Vec::new();

    // Convert to character indices to handle UTF-8 properly
    let line_chars: Vec<(usize, char)> = line.char_indices().collect();
    let query_len = query.chars().count();

    if query_len == 0 || line_chars.len() < query_len {
        return Highlight {
            line,
            highlight_ranges,
        };
    }

    let case_correctly = |input_str: &str| -> String {
        match case_sensitive {
            true => input_str.to_string(),
            false => input_str.to_lowercase(),
        }
    };
    for i in (query_len - 1)..line_chars.len() {
        let start_char_idx = i + 1 - query_len;
        let start_byte = line_chars[start_char_idx].0;
        let end_byte = if i + 1 < line_chars.len() {
            line_chars[i + 1].0
        } else {
            line.len()
        };

        let token = &line[start_byte..end_byte];

        if case_correctly(&token) == case_correctly(&query) {
            highlight_ranges.push((start_char_idx, i + 1));
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
        .map(|line| highlight_line(line, query, true))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<Highlight<'a>> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .map(|line| highlight_line(line, query, false))
        .collect()
}
pub struct Config {
    pub query: String,
    pub file: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>,
    {
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

#[cfg(test)]
mod tests {
    use super::*;
    struct Args {
        program: String,
        query: Option<String>,
        file: Option<String>,
    }
    impl Iterator for Args {
        type Item = String;
        fn next(&mut self) -> Option<Self::Item> {
            // Provide program, then query, then file, each as they are requested, consuming the Option in order.
            let next_val = if self.program.is_empty() {
                None
            } else {
                let prog = std::mem::take(&mut self.program);
                if !prog.is_empty() {
                    return Some(prog);
                }
                // else fallthrough to next
                None
            };

            if next_val.is_some() {
                return next_val;
            }

            if let Some(q) = self.query.take() {
                return Some(q);
            }
            if let Some(f) = self.file.take() {
                return Some(f);
            }
            None
        }
    }
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let results = search(query, contents);
        let lines: Vec<&str> = results.iter().map(|h| h.line).collect();
        assert_eq!(lines, vec!["safe, fast, productive."]);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        let results = search_case_insensitive(query, contents);
        let lines: Vec<&str> = results.iter().map(|h| h.line).collect();
        assert_eq!(lines, vec!["Rust:", "Trust me."]);
    }

    #[test]
    fn config_new_success() {
        let args = Args {
            program: "minigrep".to_string(),
            query: Some("foo".to_string()),
            file: Some("bar".to_string()),
        };
        let config = Config::new(args);
        assert!(config.is_ok());
        let cfg = config.unwrap();
        assert_eq!(cfg.query, "foo");
        assert_eq!(cfg.file, "bar");
    }

    #[test]
    fn config_new_missing_query() {
        let args = Args {
            program: "minigrep".to_string(),
            query: None,
            file: None,
        };
        let config = Config::new(args);
        assert!(config.is_err());
    }

    #[test]
    fn config_new_missing_file() {
        let args = Args {
            program: "minigrep".to_string(),
            query: Some("foo".to_string()),
            file: None,
        };
        let config = Config::new(args);
        assert!(config.is_err());
    }
}
