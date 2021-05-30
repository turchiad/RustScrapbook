use std::error::Error; //So we may define Box<dyn Error>
use std::fs; // To allow for file I/O

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        //Check if the arguments are formatted properly
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read file
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong when reading the file.");

    // Announce successful read
    println!("With text {}", contents);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::path::Path; //So we may check if files exist before we cleanup

    //Config tests
    #[test]
    fn new_config_not_enough_args() {
        for i in 0..2 {
            let mut args: Vec<String> = vec![];

            for j in 0..i {
                args.push(format!("{}", j));
            }

            let config = Config::new(&args);

            assert!(config.is_err())
        }
    }
    #[test]
    fn new_config_enough_args() {
        for i in 3..100 {
            let mut args: Vec<String> = vec![];

            for j in 0..i {
                args.push(format!("{}", j));
            }

            let config = Config::new(&args);

            assert!(config.is_ok())
        }
    }
    #[test]
    fn new_config_correct_assignment() {
        let args = vec![
            String::from(""),
            String::from("Query"),
            String::from("Filename"),
        ];

        let config = Config::new(&args).unwrap();

        assert_eq!(config.query, String::from("Query"));
        assert_eq!(config.filename, String::from("Filename"));
    }
    //run() tests
    #[test]
    #[should_panic]
    fn run_empty_filename() {
        let config = Config {
            query: String::from(""),
            filename: String::from(""),
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
        };

        let result = run(config);

        assert!(result.is_ok());

        fs::remove_file("test_run_real_filename_normal_file.txt").unwrap();
    }

    // // We need to run this to get rif of any files which are lingering because of a panicked
    // fn cleanup() -> Result<(), Box<dyn Error>> {
    //     let files = vec![
    //         "test_run_incorrect_filename.txt",
    //         "test_run_real_filename_empty_file.txt",
    //         "test_run_real_filename_normal_file.txt",
    //     ];

    //     for s in files {
    //         if Path::new(s).is_file() {
    //             println!("{} is a file", s);
    //             fs::remove_file(s).unwrap();
    //         }
    //     }

    //     Ok(())
    // }
}
