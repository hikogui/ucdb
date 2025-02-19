
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
    code_point_descriptions: &mut Vec<CodePointDescription>,
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
                if op(&mut code_point_descriptions[cp]).is_empty()  {
                    *op(&mut code_point_descriptions[cp]) = cap[3].to_string();
                }
            }

        } else if let Some(cap) = single_re.captures(&line) {
            // Use integers directly, char do not allow surrogates.
            let cp = usize::from_str_radix(&cap[1], 16)?;
            *op(&mut code_point_descriptions[cp]) = cap[2].to_string();

        } else if let Some(cap) = range_re.captures(&line) {
            // Use integers directly, char do not allow surrogates.
            let first_cp = usize::from_str_radix(&cap[1], 16)?;
            let last_cp = usize::from_str_radix(&cap[2], 16)?;

            for cp in first_cp..=last_cp {
                *op(&mut code_point_descriptions[cp]) = cap[3].to_string();
            }
        }
    }

    return Ok(());
}


pub fn parse_existance_column<'a>(
    url: &str,
    path: &std::path::Path,
    code_point_descriptions: &mut Vec<CodePointDescription>,
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
            *op(&mut code_point_descriptions[cp]) = true;

        } else if let Some(cap) = range_re.captures(&line) {
            // Use integers directly, char do not allow surrogates.
            let first_cp = usize::from_str_radix(&cap[1], 16)?;
            let last_cp = usize::from_str_radix(&cap[2], 16)?;

            for cp in first_cp..=last_cp {
                *op(&mut code_point_descriptions[cp]) = true;
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

pub fn parse_unicode_data_columns<'a>(
    url: &str,
    path: &std::path::Path,
    code_point_descriptions: &mut Vec<CodePointDescription>
    )
    -> Result<(), Error>
{
    // 1:code value;([0-9a-fA-F]+)
    // -:character name;
    // 2:general category;([a-zA-Z]+)
    // 3:canonical combining class;([0-9]+)
    // 4:bidirectional category;([A-Z]+)
    // 5:character decomposition mapping;([<>a-zA-Z0-9 ]+>)?
    // -:decimal digit value;
    // -:digit value;
    // -:numeric value;
    // -:unicode 1.0 name;
    // -:iso 10646 comment;
    // -:uppercase mapping;
    // -:lowercase mapping;
    // -:titlecase mapping
    let single_re = Regex::new(r"^([0-9a-fA-F]+);.*?;([a-zA-Z]+);([0-9]+);([A-Z]+);([<>a-zA-Z0-9 ]*);").unwrap();

    let file = download::download_and_open_file(&url, &path)?;
    let reader = std::io::BufReader::new(file);
    let mut first_code_value_of_range : usize = 0;
    for line_result in reader.lines() {
        let line = line_result?;

        if let Some(cap) = single_re.captures(&line) {
            // Use integers directly, char do not allow surrogates.
            let code_value = usize::from_str_radix(&cap[1], 16)?;

            if line.contains("First>:") {
                first_code_value_of_range = code_value;
            }

            code_point_descriptions[code_value].general_category = cap[2].to_string();
            code_point_descriptions[code_value].canonical_combining_class = u8::from_str_radix(&cap[3], 10).unwrap();
            code_point_descriptions[code_value].bidi_class = cap[4].to_string();
            code_point_descriptions[code_value].decomposition_type = "canonical".to_string();
            code_point_descriptions[code_value].decomposition_mapping = String::new();

            let mut decomposition = cap[5].to_string();
            while !decomposition.is_empty() {
                if decomposition.starts_with("<") {
                    let end = decomposition.find('>').unwrap();
                    let sub = &decomposition[1..end-1];
                    code_point_descriptions[code_value].decomposition_type = sub.to_string();

                    decomposition = String::from(&decomposition[end+1..]);

                } else if decomposition.starts_with(" ") {
                    decomposition = String::from(&decomposition[1..]);

                } else {
                    let end = decomposition.find(' ').unwrap_or(decomposition.len());
                    let sub = &decomposition[0..end];
                    let decomposition_code_value = u32::from_str_radix(&sub, 16)?;
                    let decomposition_cp = char::from_u32(decomposition_code_value).unwrap();
                    code_point_descriptions[code_value].decomposition_mapping.push(decomposition_cp);

                    decomposition = String::from(&decomposition[end..]);
                }
            }

            if line.contains("Last>:") {
                for i in first_code_value_of_range..code_value {
                    code_point_descriptions[i].general_category = code_point_descriptions[code_value].general_category.clone();
                    code_point_descriptions[i].canonical_combining_class = code_point_descriptions[code_value].canonical_combining_class;
                    code_point_descriptions[i].bidi_class = code_point_descriptions[code_value].bidi_class.clone();
                    code_point_descriptions[i].decomposition_type = code_point_descriptions[code_value].decomposition_type.clone();
                    code_point_descriptions[i].decomposition_mapping = code_point_descriptions[code_value].decomposition_mapping.clone();
                }
            }

        } else {
            panic!("Expecting only lines with data: '{}'", line);
        }
    }

    return Ok(());
}
