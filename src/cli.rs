/// Structure representing command-line arguments.
#[derive(Debug)]
pub struct Args {
    pub input: std::path::PathBuf,
    pub output: std::path::PathBuf,
    pub threads: Option<usize>,
}

impl Args {
    /// Parses command-line arguments using `clap`.
    ///
    /// # Returns
    /// * `Args` - Struct containing parsed arguments.
    ///
    /// # Errors
    /// * If required arguments are missing or invalid.    
    pub fn parse() -> Self {
        let matches = clap::Command::new("csv_to_parquet")
            .version("0.1.0")
            .author("AndyDar")
            .about("Convert CSV/TXT files to Parquet")
            .arg(
                clap::Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("Path to input directory with CSV/TXT files")
                    .required(true)
                    .num_args(1),
            )
            .arg(
                clap::Arg::new("output")
                .short('o')
                .long("output")
                .help("Path to output directory for Parquet files")
                .required(true)
                .num_args(1),
            )
            .arg(
                clap::Arg::new("threads")
                .short('t')
                .long("threads")
                .help("Number of threads to use (default: all available)")
                .num_args(1)
                .value_parser(clap::builder::ValueParser::new(parse_usize_positive)),
            )
            .get_matches();

        Args {
            input: std::path::PathBuf::from(matches.get_one::<String>("input").unwrap()),
            output: std::path::PathBuf::from(matches.get_one::<String>("output").unwrap()),
            threads: matches.get_one::<usize>("threads").cloned(),
        }
    }
}

/// Validates that the number of threads is a positive integer.
///
/// # Arguments
/// * `s` - String representation of the number of threads.
///
/// # Returns
/// * `Result<usize>` - Validated number of threads.
fn parse_usize_positive(s: &str) -> Result<usize, String> {
    match s.parse::<usize>() {
        Ok(0) => Err("Must be a positive integer".to_string()),
        Ok(n) => Ok(n),
        Err(e) => Err(format!("Not a valid number: {}", e)),
    }
}
