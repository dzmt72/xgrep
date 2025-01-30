use std::{fs::File, io::BufReader};
use regex::Regex;

fn parse_file_re(reader: BufReader<&File>, re: Regex) -> Vec<(usize, String)> {
    let result: Vec<(usize, String)> = Vec::new();

    unimplemented!()
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("demo")?;

    let re = Regex::new(r"hey").unwrap();
    let reader = BufReader::new(&file);

    let result: Vec<(usize, String)> = parse_file_re(reader, re);

    dbg!(result);

    Ok(())
}
