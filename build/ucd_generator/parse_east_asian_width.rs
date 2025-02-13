
//use std::fs::File;
use std::io::BufRead;
use crate::ucd_generator::CodePointDescription;
use crate::ucd_generator::download;
use regex::Regex;


pub fn parse_east_asian_width(ucd_base_url: &str, ucd_version: &str, data_dir: &std::path::Path, descriptions : &mut Vec<CodePointDescription>)
    -> Result<(), String>
{
    let missing_re = match Regex::new(r"^#\s*@missing:\s*([0-9a-fA-F]+)\.\.([0-9a-fA-F]+)\s*;\s*([a-zA-Z]+)") {
        Ok(x) => x,
        Err(e) => return Err(format!("Error creating regex: {}", e)),
    };

    let single_re = match Regex::new(r"^([0-9a-fA-F]+)\s*;\s*([a-zA-Z]+)") {
        Ok(x) => x,
        Err(e) => return Err(format!("Error creating regex: {}", e)),
    };

    let range_re = match Regex::new(r"^([0-9a-fA-F]+)\.\.([0-9a-fA-F]+)\s*;\s*([a-zA-Z]+)") {
        Ok(x) => x,
        Err(e) => return Err(format!("Error creating regex:  {}", e)),
    };

    let url = format!("{}/{}/ucd/EastAsianWidth.txt", &ucd_base_url, &ucd_version);
    let path = data_dir.join(&ucd_version).join("ucd").join("EastAsianWidth.txt");

    let file = download::download_and_open_file(&url, &path)?;
    let reader = std::io::BufReader::new(file);
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(x) => x,
            Err(e) => return Err(format!("Error reading file: {}", e)),
        };

        if let Some(cap) = missing_re.captures(&line) {
            // Use integers directly, char do not allow surrogates.
            let first_cp = usize::from_str_radix(&cap[1], 16).unwrap();
            let last_cp = usize::from_str_radix(&cap[2], 16).unwrap();

            for cp in first_cp..=last_cp {
                if descriptions[cp].east_asian_width.is_empty()  {
                    descriptions[cp].east_asian_width = cap[3].to_string();
                }
            }

        } else if let Some(cap) = single_re.captures(&line) {
            // Use integers directly, char do not allow surrogates.
            let cp = usize::from_str_radix(&cap[1], 16).unwrap();
            descriptions[cp].east_asian_width = cap[2].to_string();

        } else if let Some(cap) = range_re.captures(&line) {
            // Use integers directly, char do not allow surrogates.
            let first_cp = usize::from_str_radix(&cap[1], 16).unwrap();
            let last_cp = usize::from_str_radix(&cap[2], 16).unwrap();

            for cp in first_cp..=last_cp {
                descriptions[cp].east_asian_width = cap[2].to_string();
            }
        }
    }

    return Ok(());
}

