use std::{collections::HashMap, fs::{metadata, File}, io::{BufRead, BufReader}};
use regex::Regex;
use clap::Parser;

use colored::Colorize;

#[derive(Parser)]
struct Args {
    #[arg()]
    regex: String,

    #[arg(num_args=0..=10)]
    paths: Vec<String>
}

fn highlight_mathces(str: String, re: &Regex) -> String {
    let highlight = re.replace_all(&str, |cap: &regex::Captures| {
        cap.get(0).unwrap().as_str().red().to_string()
    });

    highlight.to_string()
}

fn parse_files(
    paths: Vec<String>,
    re: &Regex,
) -> Result<HashMap<String, Vec<(usize, String)>>, std::io::Error> {
    let mut matches: HashMap<String, Vec<(usize, String)>> = HashMap::new();

    for path in paths {
        let meta = metadata(&path)?;

        if meta.is_dir() {
            unimplemented!()

            /*
            files_in_dir
            parse_files(files_in_dir)
            */
        } else if meta.is_file() {
            let file = File::open(&path)?;
            let reader = BufReader::new(&file);

            let res: Vec<(usize, String)> = parse_content(reader, &re);

            matches.insert(path, res);
        } else {
            println!("undefined filetype");
        }
   } 

    Ok(matches)
}

fn parse_content(reader: BufReader<&File>, re: &Regex) -> Vec<(usize, String)> {
    let mut result: Vec<(usize, String)> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        if let Ok(l) = line {
            if re.is_match(&l) {
                result.push((i + 1, highlight_mathces(l, &re)));
            }
        } else {
            eprintln!("error reading line {}: {}", i, line.unwrap_err());
        }
    }

    result 
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let re = Regex::new(&args.regex).unwrap();

    let matches = parse_files(args.paths, &re)?;

    for (filename, v) in matches {
        for (i, str) in v {
            println!("{}\t{}\t{}", i, str, filename);
        }
    }

    Ok(())
}
