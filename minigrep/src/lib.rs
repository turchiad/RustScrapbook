use std::env;
use std::error::Error; //So we may define Box<dyn Error>
use std::fs; // To allow for file I/O // To allow for the environment variable to be adjusted

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Discard the first argument

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read file
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong when reading the file.");

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
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
    contents
        .lines()
        .filter(|line| {
            line.to_lowercase()
                .contains(&String::from(query).to_lowercase())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::path::Path; //So we may check if files exist before we cleanup

    //Config tests
    // #[test]
    // fn new_config_not_enough_args() {
    //     for i in 0..2 {
    //         let mut args: Vec<String> = vec![];

    //         for j in 0..i {
    //             args.push(format!("{}", j));
    //         }

    //         let config = Config::new(&args.iter());

    //         assert!(config.is_err())
    //     }
    // }
    // #[test]
    // fn new_config_enough_args() {
    //     for i in 3..100 {
    //         let mut args: Vec<String> = vec![];

    //         for j in 0..i {
    //             args.push(format!("{}", j));
    //         }

    //         let config = Config::new(&args);

    //         assert!(config.is_ok())
    //     }
    // }
    // #[test]
    // fn new_config_correct_assignment() {
    //     let args = vec![
    //         String::from(""),
    //         String::from("Query"),
    //         String::from("Filename"),
    //     ];

    //     let config = Config::new(&args).unwrap();

    //     assert_eq!(config.query, String::from("Query"));
    //     assert_eq!(config.filename, String::from("Filename"));
    // }
    // //run() tests
    #[test]
    #[should_panic]
    fn run_empty_filename() {
        let config = Config {
            query: String::from(""),
            filename: String::from(""),
            case_sensitive: false,
        };

        let result = run(config);

        assert!(result.is_err(), "{:?}", result)
    }
    #[test]
    #[should_panic]
    fn run_incorrect_filename() {
        //fs::write("test_run_incorrect_filename.txt", "test").unwrap();

        let config = Config {
            query: String::from(""),
            filename: String::from("test_run_incorrect_filename.txt-a"),
            case_sensitive: false,
        };

        let result = run(config);

        assert!(result.is_err(), "{:?}", result);

        //fs::remove_file("test_run_incorrect_filename.txt").unwrap();
    }
    #[test]
    fn run_real_filename_empty_file() {
        fs::File::create("test_run_real_filename_empty_file.txt").unwrap();

        let config = Config {
            query: String::from("test"),
            filename: String::from("test_run_real_filename_empty_file.txt"),
            case_sensitive: false,
        };

        let result = run(config);

        assert!(result.is_ok());

        fs::remove_file("test_run_real_filename_empty_file.txt").unwrap();
    }
    #[test]
    fn run_real_filename_normal_file() {
        fs::write("test_run_real_filename_normal_file.txt", "test").unwrap();

        let config = Config {
            query: String::from("test"),
            filename: String::from("test_run_real_filename_normal_file.txt"),
            case_sensitive: false,
        };

        let result = run(config);

        assert!(result.is_ok());

        fs::remove_file("test_run_real_filename_normal_file.txt").unwrap();
    }
    //search() tests
    #[test]
    fn search_one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
    #[test]
    fn search_one_result_case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
    #[test]
    fn search_multiple_results() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nI like ducts.";

        assert_eq!(
            vec!["safe, fast, productive.", "I like ducts."],
            search(query, contents)
        )
    }
    #[test]
    fn search_multiple_results_case_sensitive() {
        let query = "duct";
        let contents =
            "Rust:\nsafe, fast, productive.\nPick three.\nDucts are my favorite.\nI like ducts.";

        assert_eq!(
            vec!["safe, fast, productive.", "I like ducts."],
            search(query, contents)
        )
    }
    #[test]
    fn search_case_insensitive_one_result() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents))
    }
    #[test]
    fn search_case_insensitive_multiple_results() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
