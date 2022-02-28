use clap::{Arg, Command};

fn main() {
    // println!("{:?}", std::env::args());
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("rnella01")
        .about("Rust Echo utility")
        .args(vec![
            Arg::new("text")
                .value_name("TEXT")
                .help("Input Text")
                .allow_invalid_utf8(true)
                .required(true)
                .min_values(1),
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .takes_value(false),
        ])
        .get_matches();
    // println!("{:#?}", matches);
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline {  ""  } else { "\n" });
}
