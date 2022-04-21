use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("rnella01")
        .about("catr - cat utility in Rust")
        .args(vec![
            Arg::new("files")
                .value_name("FILES")
                .help("files to cat")
                .multiple_values(true)
                .default_value("-")
                .allow_invalid_utf8(true),
            Arg::new("number_lines")
                .short('n')
                .long("number-lines")
                .help("number all lines")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines"),
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank-lines")
                .help("number non-blank lines only")
                .takes_value(false),
        ])
        .get_matches();
    let files = matches.values_of_lossy("files").unwrap();
    let number_lines = matches.is_present("number_lines");
    let number_nonblank_lines = matches.is_present("number_nonblank_lines");
    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    // dbg!(config);
    for filename in config.files {
        match open_file(&filename) {
            Err(err) => {
                eprintln!("Unable to open file {}. Error: {}", filename, err);
            }
            Ok(mut bufRead) => {
                for line in bufRead.lines() {
                    println!("{}", line?);
                }
            }
        }
    }
    Ok(())
}

fn open_file(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
