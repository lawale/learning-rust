use regex::Regex;
use clap::{Command, Arg};

fn main() {

    let matches = Command::new("grep-lite")// requires `cargo` feature
        .about("searches for pattern")
        .version("0.1.0")
        .arg(Arg::new("pattern")
            .help("The pattern to search for")
            .required(false))
        .get_matches();

    let pattern: &String;
    match matches.get_one("pattern") {
        Some(p) => pattern = p,
        None => panic!("Pattern not found")
    }

    let re = Regex::new(pattern).unwrap();

    let quote = "\

Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        let contain_substring = re.find(line);
        match contain_substring {
            Some(_) => {
                println!("{}", line);
            },
            None => ()
        }
    }
}