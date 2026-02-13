use std::{env::Args, error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn file_path(&self) -> &String {
        &self.file_path
    }

    pub fn ignore_case(&self) -> &bool {
        &self.ignore_case
    }

    pub fn build(mut args: Args, ignore_case: bool) -> Result<Config, &'static str> {
        args.next(); // Skip program name

        let query = args.next().ok_or("Please enter query as 1st argument")?;

        let file_path = args
            .next()
            .ok_or("Please enter file path as 2nd argument")?;

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path())?;

    let matched_lines = match config.ignore_case() {
        true => search_case_insensitive(config.query(), &contents),
        false => search(config.query(), &contents),
    };

    for matched_line in matched_lines {
        println!("{matched_line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lowercase_query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&lowercase_query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn run_prints_matched_lines() {
    //     let args = [
    //         String::from("command_name"),
    //         String::from("banish"),
    //         String::from("test-poem.txt"),
    //     ];

    //     let config = Config::build(&args);
    // }

    #[test]
    fn case_insensitive_search_finds_one_line() {
        let query = "Banish";
        // Backslash tells Rust that it should ignore the first New line
        let contents = "\
            Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.
        ";

        assert_eq!(
            vec!["They'd banish us, you know."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive_search_finds_multiple_lines() {
        let query = "Banish";
        // Backslash tells Rust that it should ignore the first New line
        let contents = "\
            Are you baNiShed, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.
        ";

        assert_eq!(
            vec!["Are you baNiShed, too?", "They'd banish us, you know."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive_search_returns_empty_list_if_no_results_found() {
        let query = "banish";
        // Backslash tells Rust that it should ignore the first New line
        let contents = "\
            Are you nobody, too?
Then there's a pair of us - don't tell!
They'd vanish us, you know.
        ";

        let empty_vec: Vec<&str> = Vec::new();
        assert_eq!(empty_vec, search_case_insensitive(query, contents));
    }

    #[test]
    fn case_sensitive_search_finds_one_line() {
        let query = "banish";
        // Backslash tells Rust that it should ignore the first New line
        let contents = "\
            Are you Banished, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.
        ";

        assert_eq!(vec!["They'd banish us, you know."], search(query, contents));
    }

    #[test]
    fn case_sensitive_search_finds_multiple_lines() {
        let query = "banish";
        // Backslash tells Rust that it should ignore the first New line
        let contents = "\
            Are you banished, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.
        ";

        assert_eq!(
            vec!["Are you banished, too?", "They'd banish us, you know."],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive_search_returns_empty_list_if_no_results_found() {
        let query = "banish";
        // Backslash tells Rust that it should ignore the first New line
        let contents = "\
            Are you nobody, too?
Then there's a pair of us - don't tell!
They'd vanish us, you know.
        ";

        let empty_vec: Vec<&str> = Vec::new();
        assert_eq!(empty_vec, search(query, contents));
    }
}
