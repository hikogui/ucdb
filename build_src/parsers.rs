
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


pub fn parse_existance_column<'a>(
    url: &str,
    path: &std::path::Path,
    column: &mut Vec<CodePointDescription>,
    op : impl Fn(&mut CodePointDescription) -> &mut bool
    )
    -> Result<(), Error>
{
    let single_re = Regex::new(r"^([0-9a-fA-F]+)").unwrap();
    let range_re = Regex::new(r"^([0-9a-fA-F]+)\.\.([0-9a-fA-F]+)").unwrap();

    let file = download::download_and_open_file(&url, &path)?;
    let reader = std::io::BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result?;

        if let Some(cap) = single_re.captures(&line) {
            // Use integers directly, char do not allow surrogates.
            let cp = usize::from_str_radix(&cap[1], 16)?;
            *op(&mut column[cp]) = true;

        } else if let Some(cap) = range_re.captures(&line) {
            // Use integers directly, char do not allow surrogates.
            let first_cp = usize::from_str_radix(&cap[1], 16)?;
            let last_cp = usize::from_str_radix(&cap[2], 16)?;

            for cp in first_cp..=last_cp {
                *op(&mut column[cp]) = true;
            }
        }
    }

    return Ok(());
}

pub fn parse_prop_list_columns<'a>(
    url: &str,
    path: &std::path::Path,
    code_point_descriptions: &mut Vec<CodePointDescription>
    )
    -> Result<(), Error>
{
    let single_re = Regex::new(r"^([0-9a-fA-F]+)\s*;\s*([a-zA-Z_]+)").unwrap();
    let range_re = Regex::new(r"^([0-9a-fA-F]+)\.\.([0-9a-fA-F]+)\s*;\s*([a-zA-Z_]+)").unwrap();

    let file = download::download_and_open_file(&url, &path)?;
    let reader = std::io::BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result?;

        let first_cp : usize;
        let last_cp : usize;
        let property_name : String;

        if let Some(cap) = single_re.captures(&line) {
            // Use integers directly, char do not allow surrogates.
            first_cp = usize::from_str_radix(&cap[1], 16)?;
            last_cp = first_cp;
            property_name = cap[2].to_string();

        } else if let Some(cap) = range_re.captures(&line) {
            // Use integers directly, char do not allow surrogates.
            first_cp = usize::from_str_radix(&cap[1], 16)?;
            last_cp = usize::from_str_radix(&cap[2], 16)?;
            property_name = cap[3].to_string();

        } else {
            continue;
        }

        for cp in first_cp..=last_cp {
            match property_name.as_str() {
                "White_Space" => code_point_descriptions[cp].white_space = true,
                "Bidi_Control" => code_point_descriptions[cp].bidi_control = true,
                "Join_Control" => code_point_descriptions[cp].join_control = true,
                "Dash" => code_point_descriptions[cp].dash = true,
                "Hyphen" => code_point_descriptions[cp].hyphen = true,
                "Quotation_Mark" => code_point_descriptions[cp].quotation_mark = true,
                "Terminal_Punctuation" => code_point_descriptions[cp].terminal_punctuation = true,
                "Other_Math" => code_point_descriptions[cp].other_math = true,
                "Hex_Digit" => code_point_descriptions[cp].hex_digit = true,
                "ASCII_Hex_Digit" => code_point_descriptions[cp].ascii_hex_digit = true,
                "Other_Alphabetic" => code_point_descriptions[cp].other_alphabetic = true,
                "Ideographic" => code_point_descriptions[cp].ideographic = true,
                "Diacritic" => code_point_descriptions[cp].diacritic = true,
                "Extender" => code_point_descriptions[cp].extender = true,
                "Other_Lowercase" => code_point_descriptions[cp].other_lowercase = true,
                "Other_Uppercase" => code_point_descriptions[cp].other_uppercase = true,
                "Noncharacter_Code_Point" => code_point_descriptions[cp].noncharacter_code_point = true,
                "Other_Grapheme_Extend" => code_point_descriptions[cp].other_grapheme_extend = true,
                "IDS_Unary_Operator" => code_point_descriptions[cp].ids_unary_operator = true,
                "IDS_Binary_Operator" => code_point_descriptions[cp].ids_binary_operator = true,
                "IDS_Trinary_Operator" => code_point_descriptions[cp].ids_trinary_operator = true,
                "Radical" => code_point_descriptions[cp].radical = true,
                "Unified_Ideograph" => code_point_descriptions[cp].unified_ideograph = true,
                "Other_Default_Ignorable_Code_Point" => code_point_descriptions[cp].other_default_ignorable_code_point = true,
                "Deprecated" => code_point_descriptions[cp].deprecated = true,
                "Soft_Dotted" => code_point_descriptions[cp].soft_dotted = true,
                "Logical_Order_Exception" => code_point_descriptions[cp].logical_order_exception = true,
                "Other_ID_Start" => code_point_descriptions[cp].other_id_start = true,
                "Other_ID_Continue" => code_point_descriptions[cp].other_id_continue = true,
                "ID_Compat_Math_Continue" => code_point_descriptions[cp].id_compat_math_continue = true,
                "ID_Compat_Math_Start" => code_point_descriptions[cp].id_compat_math_start = true,
                "Sentence_Terminal" => code_point_descriptions[cp].sentence_terminal = true,
                "Variation_Selector" => code_point_descriptions[cp].variation_selector = true,
                "Pattern_White_Space" => code_point_descriptions[cp].pattern_white_space = true,
                "Pattern_Syntax" => code_point_descriptions[cp].pattern_syntax = true,
                "Prepended_Concatenation_Mark" => code_point_descriptions[cp].prepended_concatenation_mark = true,
                "Regional_Indicator" => code_point_descriptions[cp].regional_indicator = true,
                "Modifier_Combining_Mark" => code_point_descriptions[cp].modifier_combining_mark = true,
                _ => panic!("Unknown property {}", property_name),
            }
        }
    }

    return Ok(());
}

