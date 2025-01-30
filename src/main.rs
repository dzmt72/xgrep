use std::{fs::File, io::{BufRead, BufReader}};
use regex::Regex;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg()]
    regex: String,

    #[arg()]
    path: String
}

fn parse_file_re(reader: BufReader<&File>, re: Regex) -> Vec<(usize, String)> {
    let mut result: Vec<(usize, String)> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        if let Ok(l) = line {
            if re.is_match(&l) {
                result.push((i + 1, l));
            }
        } else {
            eprintln!("error reading line {}: {}", i, line.unwrap_err());
        }
    }

    result 
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let file = File::open(&args.path)?;

    let re = Regex::new(&args.regex).unwrap();
    let reader = BufReader::new(&file);

    let result: Vec<(usize, String)> = parse_file_re(reader, re);

    for (i, l) in result {
        println!("{}\t{}\t{}", i, l, &args.path);
    }

    Ok(())
}
