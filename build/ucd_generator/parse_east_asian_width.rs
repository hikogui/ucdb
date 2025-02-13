
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use crate::ucd_generator::CodePointDescription;

enum LineResult {
    None,
    End,
    Single(char, String),
    Range(char, char, String),
    Missing(String),
}

pub fn scan_east_asian_width(line : &str) -> LineResult {

}

pub fn parse_east_asian_width(ucd_version : &str, descriptions : &mut Vec<CodePointDescription>)
    -> Result<(), String> {

    let file = File::open("foo.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    return Ok(());
}

