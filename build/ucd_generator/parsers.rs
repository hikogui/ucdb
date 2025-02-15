
use crate::ucd_generator::CodePointDescription;
use crate::ucd_generator::download;
use std::io::BufRead;
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to write")]
    IO(#[from] std::io::Error),
    #[error("Failed to download")]
    Download(#[from] download::Error),
    #[error("Failed to parse integer from file")]
    Parse(#[from] std::num::ParseIntError),
}

pub fn parse_east_asian_width(ucd_base_url: &str, ucd_version: &str, data_dir: &std::path::Path, descriptions : &mut Vec<CodePointDescription>)
    -> Result<(), Error>
{
    let missing_re = Regex::new(r"^#\s*@missing:\s*([0-9a-fA-F]+)\.\.([0-9a-fA-F]+)\s*;\s*([a-zA-Z]+)").unwrap();
    let single_re = Regex::new(r"^([0-9a-fA-F]+)\s*;\s*([a-zA-Z]+)").unwrap();
    let range_re = Regex::new(r"^([0-9a-fA-F]+)\.\.([0-9a-fA-F]+)\s*;\s*([a-zA-Z]+)").unwrap();

    let url = format!("{}/{}/ucd/EastAsianWidth.txt", &ucd_base_url, &ucd_version);
    let path = data_dir.join(&ucd_version).join("ucd").join("EastAsianWidth.txt");

    let file = download::download_and_open_file(&url, &path)?;
    let reader = std::io::BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result?;

        if let Some(cap) = missing_re.captures(&line) {
            // Use integers directly, char do not allow surrogates.
            let first_cp = usize::from_str_radix(&cap[1], 16)?;
            let last_cp = usize::from_str_radix(&cap[2], 16)?;

            for cp in first_cp..=last_cp {
                if descriptions[cp].east_asian_width.is_empty()  {
                    descriptions[cp].east_asian_width = cap[3].to_string();
                }
            }

        } else if let Some(cap) = single_re.captures(&line) {
            // Use integers directly, char do not allow surrogates.
            let cp = usize::from_str_radix(&cap[1], 16)?;
            descriptions[cp].east_asian_width = cap[2].to_string();

        } else if let Some(cap) = range_re.captures(&line) {
            // Use integers directly, char do not allow surrogates.
            let first_cp = usize::from_str_radix(&cap[1], 16)?;
            let last_cp = usize::from_str_radix(&cap[2], 16)?;

            for cp in first_cp..=last_cp {
                descriptions[cp].east_asian_width = cap[3].to_string();
            }
        }
    }

    return Ok(());
}

