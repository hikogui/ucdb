
use crate::build_src::CodePointDescription;
use crate::build_src::download;
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

pub fn parse_single_column<'a>(
    url: &str,
    path: &std::path::Path,
    column: &mut Vec<CodePointDescription>,
    op : impl Fn(&mut CodePointDescription) -> &mut String
    )
    -> Result<(), Error>
{
    let missing_re = Regex::new(r"^#\s*@missing:\s*([0-9a-fA-F]+)\.\.([0-9a-fA-F]+)\s*;\s*([a-zA-Z_]+)").unwrap();
    let single_re = Regex::new(r"^([0-9a-fA-F]+)\s*;\s*([a-zA-Z_]+)").unwrap();
    let range_re = Regex::new(r"^([0-9a-fA-F]+)\.\.([0-9a-fA-F]+)\s*;\s*([a-zA-Z_]+)").unwrap();

    let file = download::download_and_open_file(&url, &path)?;
    let reader = std::io::BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result?;

        if let Some(cap) = missing_re.captures(&line) {
            // Use integers directly, char do not allow surrogates.
            let first_cp = usize::from_str_radix(&cap[1], 16)?;
            let last_cp = usize::from_str_radix(&cap[2], 16)?;

            for cp in first_cp..=last_cp {
                if op(&mut column[cp]).is_empty()  {
                    *op(&mut column[cp]) = cap[3].to_string();
                }
            }

        } else if let Some(cap) = single_re.captures(&line) {
            // Use integers directly, char do not allow surrogates.
            let cp = usize::from_str_radix(&cap[1], 16)?;
            *op(&mut column[cp]) = cap[2].to_string();

        } else if let Some(cap) = range_re.captures(&line) {
            // Use integers directly, char do not allow surrogates.
            let first_cp = usize::from_str_radix(&cap[1], 16)?;
            let last_cp = usize::from_str_radix(&cap[2], 16)?;

            for cp in first_cp..=last_cp {
                *op(&mut column[cp]) = cap[3].to_string();
            }
        }
    }

    return Ok(());
}


