use std::{fs, process};

pub struct UnsafeArgs {
    args: Result<Args, &'static str>,
    ignore_case: bool,
}

pub struct Args {
    query: String,
    file_path: String,
}

pub struct App {
    args: Args,
}

pub struct Grep {
    query: String,
    ignore_case: bool,
}

impl UnsafeArgs {
    pub fn new(args_vec: &[String], ignore_case: bool) -> UnsafeArgs {
        UnsafeArgs {
            args: Args::new(args_vec),
            ignore_case,
        }
    }
}

impl<'a> Args {
    pub fn new(args_vec: &[String]) -> Result<Args, &'static str> {
        if args_vec.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Args {
            query: args_vec[1].clone(),
            file_path: args_vec[2].clone(),
        })
    }
}

impl App {
    pub fn new(unsafe_args: UnsafeArgs) -> App {
        App {
            args: unsafe_args.args.unwrap_or_else(|err| {
                eprintln!("Problem parsing arguments: {}", err);
                process::exit(1);
            }),
        }
    }

    pub fn run(&self) {
        let contents = fs::read_to_string(&self.args.file_path).unwrap_or_else(|err| {
            eprintln!("Application error: {err}");
            process::exit(1);
        });
        let grep = Grep {
            query: self.args.query.clone(),
            ignore_case: false,
        };
        let results = grep.search(&contents);
        if results.is_empty() {
            println!("No matches found.");
        } else {
            for line in results {
                println!("{}", line);
            }
        }
    }
}

impl Grep {
    pub fn search<'a>(&self, contents: &'a str) -> Vec<&'a str> {
        let query = if self.ignore_case {
            &self.query.to_lowercase()
        } else {
            &self.query
        };
        contents
            .lines()
            .filter(|line| {
                return if self.ignore_case {
                    line.to_lowercase().contains(query)
                } else {
                    line.contains(query)
                };
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Grep;

    #[test]
    fn one_result() {
        let subject = Grep {
            query: "duct".to_string(),
            ignore_case: false,
        };
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], subject.search(contents));
    }

    #[test]
    fn one_result_case_insensitive() {
        let subject = Grep {
            query: "pick".to_string(),
            ignore_case: true,
        };
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Pick three."], subject.search(contents));
    }
}
