use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use clap::{Command, Arg};

fn main() {

    let matches = Command::new("grep-lite")// requires `cargo` feature
        .about("searches for pattern")
        .version("0.1.0")
        .arg(Arg::new("pattern")
            .long("pattern")
            .short('p')
            .help("The pattern to search for")
            .required(true))
        .arg(Arg::new("file")
            .long("file")
            .short('f')
            .help("The input file to search")
            .required(false))
        .get_matches();

    let std_flag = &"-".to_string();

    let file_name = matches.get_one("file").unwrap_or(std_flag);

    let pattern: &String = matches.get_one("pattern").unwrap_or_else(|| panic!("Pattern Required"));

    let re = Regex::new(pattern).unwrap();

    if file_name == "-" {
        let stdin = std::io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    }else {
        let file = File::open(file_name).unwrap_or_else(|_| panic!("File Not Found"));

        let reader = BufReader::new(file);

        process_lines(reader, re);
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, regex: Regex) {
    for _line in reader.lines(){
        let line = _line.unwrap();
        let contain_substring = regex.find(&line);
        match contain_substring {
            Some(_) => {
                println!("{}", line);
            },
            None => ()
        }
    }
}