
//use std::io::File;
//use std::io::BufReader;
use crate::ucd_generator::CodePointDescription;
//use crate::build::download;

enum LineResult {
    None,
    End,
    Single(char, String),
    Range(char, char, String),
    Missing(String),
}

pub fn scan_east_asian_width(line : &str) -> LineResult {
    return LineResult::None;
}

pub fn parse_east_asian_width(ucd_base_url: &str, ucd_version: &str, data_dir: &std::path::Path, descriptions : &mut Vec<CodePointDescription>)
    -> Result<(), String>
{
    let url = format!("{}/{}/ucd/EastAsianWidth.txt", &ucd_base_url, &ucd_version);
    let path = data_dir.join(&ucd_version).join("ucd").join("EastAsianWidth.txt");

    let file = download::download_and_open_file(&url, &path)?;

    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        match scan_east_asian_width(&line) {
            LineResult::None => (),
            LineResult::End  => break,
            LineResult::Single(_cp, _value) => {
                descriptions[_cp].east_asian_width = _value;
            },
            LineResult::Range(_start, _end, _value) => {
                for cp in _start..=_end  {
                    descriptions[cp].east_asian_width = _value.clone();
                }
            },
            LineResult::Missing(_value) => {
                for cp in '\0'..='\u{10ffff}' {
                    if descriptions[cp].east_asian_width.is_none()  {
                        descriptions[cp].east_asian_width = _value.clone();
                    }
                }
            },
        }
    }

    return Ok(());
}

