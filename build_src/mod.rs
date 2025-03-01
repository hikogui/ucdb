mod code_point_description;
mod column;
mod download;
mod generators;
mod parsers;

use code_point_description::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to write")]
    IO(#[from] std::io::Error),
    #[error("Failed to format")]
    Generator(#[from] generators::Error),
    #[error("Failed to parse")]
    Parser(#[from] parsers::Error),
}

fn generate_enum_table<'a>(
    code_dir: &std::path::Path,
    name: &str,
    mut enum_table: Vec<String>,
    descriptions: &'a Vec<CodePointDescription>,
    op: impl Fn(&'a CodePointDescription) -> &'a String,
) -> Result<(), Error> {
    let column = column::map_str_to_int(&mut enum_table, descriptions, op);

    let (dedup, dedup_bits, index, index_bits, chunk_size) = column::dedup_best_fit(&column);

    generators::generate_enum_table(&code_dir, name, &enum_table, &dedup, dedup_bits, &index, index_bits, chunk_size)?;

    return Ok(());
}

fn generate_bool_table<'a>(
    code_dir: &std::path::Path,
    name: &str,
    descriptions: &'a Vec<CodePointDescription>,
    op: impl Fn(&'a CodePointDescription) -> bool
) -> Result<(), Error> {
    let column = column::map_bool_to_int(descriptions, op);

    let (dedup, dedup_bits, index, index_bits, chunk_size) = column::dedup_best_fit(&column);
    assert!(dedup_bits == 1);

    generators::generate_bool_table(&code_dir, name, &dedup, &index, index_bits, chunk_size)?;

    return Ok(());
}

fn generate_char_table<'a>(
    code_dir: &std::path::Path,
    name: &str,
    descriptions: &'a Vec<CodePointDescription>,
    op: impl Fn(&'a CodePointDescription) -> &'a Option<char>,
) -> Result<(), Error> {
    let column = column::map_char_to_int(descriptions, op);

    let (dedup, dedup_bits, index, index_bits, chunk_size) = column::dedup_best_fit(&column);

    generators::generate_char_table(&code_dir, name, &dedup, dedup_bits, &index, index_bits, chunk_size)?;

    return Ok(());
}

pub fn build(ucd_base_url: &str, ucd_version: &str, data_dir: &std::path::Path, code_dir: &std::path::Path) -> Result<(), Error> {
    let mut descriptions = Vec::<CodePointDescription>::with_capacity(0x110000);
    descriptions.resize(0x110000, CodePointDescription::new());

    parsers::parse_single_column(
        &format!("{}/{}/ucd/EastAsianWidth.txt", &ucd_base_url, &ucd_version),
        &data_dir.join(&ucd_version).join("ucd").join("EastAsianWidth.txt"),
        &mut descriptions,
        |x| &mut x.east_asian_width,
    )?;

    parsers::parse_single_column(
        &format!("{}/{}/ucd/LineBreak.txt", &ucd_base_url, &ucd_version),
        &data_dir.join(&ucd_version).join("ucd").join("LineBreak.txt"),
        &mut descriptions,
        |x| &mut x.line_break,
    )?;

    parsers::parse_single_column(
        &format!("{}/{}/ucd/auxiliary/WordBreakProperty.txt", &ucd_base_url, &ucd_version),
        &data_dir
            .join(&ucd_version)
            .join("ucd")
            .join("auxiliary")
            .join("WordBreakProperty.txt"),
        &mut descriptions,
        |x| &mut x.word_break,
    )?;

    parsers::parse_single_column(
        &format!("{}/{}/ucd/auxiliary/SentenceBreakProperty.txt", &ucd_base_url, &ucd_version),
        &data_dir
            .join(&ucd_version)
            .join("ucd")
            .join("auxiliary")
            .join("SentenceBreakProperty.txt"),
        &mut descriptions,
        |x| &mut x.sentence_break,
    )?;

    parsers::parse_single_column(
        &format!("{}/{}/ucd/auxiliary/GraphemeBreakProperty.txt", &ucd_base_url, &ucd_version),
        &data_dir
            .join(&ucd_version)
            .join("ucd")
            .join("auxiliary")
            .join("GraphemeBreakProperty.txt"),
        &mut descriptions,
        |x| &mut x.grapheme_break,
    )?;

    parsers::parse_single_column(
        &format!("{}/{}/ucd/Scripts.txt", &ucd_base_url, &ucd_version),
        &data_dir.join(&ucd_version).join("ucd").join("Scripts.txt"),
        &mut descriptions,
        |x| &mut x.script,
    )?;

    parsers::parse_existance_column(
        &format!("{}/{}/ucd/CompositionExclusions.txt", &ucd_base_url, &ucd_version),
        &data_dir.join(&ucd_version).join("ucd").join("CompositionExclusions.txt"),
        &mut descriptions,
        |x| &mut x.composition_exclusion,
    )?;

    parsers::parse_prop_list_columns(
        &format!("{}/{}/ucd/PropList.txt", &ucd_base_url, &ucd_version),
        &data_dir.join(&ucd_version).join("ucd").join("PropList.txt"),
        &mut descriptions,
    )?;

    parsers::parse_unicode_data_columns(
        &format!("{}/{}/ucd/UnicodeData.txt", &ucd_base_url, &ucd_version),
        &data_dir.join(&ucd_version).join("ucd").join("UnicodeData.txt"),
        &mut descriptions,
    )?;


    generate_enum_table(code_dir, "east_asian_width", vec!["N".to_string()], &descriptions, |x| &x.east_asian_width)?;
    generate_enum_table(code_dir, "line_break", vec!["XX".to_string()], &descriptions, |x| &x.line_break)?;
    generate_enum_table(code_dir, "word_break", vec!["Other".to_string()], &descriptions, |x| &x.word_break)?;
    generate_enum_table(code_dir, "sentence_break", vec!["Other".to_string()], &descriptions, |x| &x.sentence_break)?;
    generate_enum_table(code_dir, "grapheme_break", vec!["Other".to_string()], &descriptions, |x| &x.grapheme_break)?;
    generate_enum_table(code_dir, "script", vec!["Unknown".to_string()], &descriptions, |x| &x.script)?;
    generate_enum_table(code_dir, "general_category", vec!["Cn".to_string()], &descriptions, |x| &x.general_category)?;

    generate_bool_table(code_dir, "composition_exclusion", &descriptions, |x| x.composition_exclusion)?;
    generate_bool_table(code_dir, "white_space", &descriptions, |x| x.white_space)?;
    generate_bool_table(code_dir, "bidi_control", &descriptions, |x| x.bidi_control)?;
    generate_bool_table(code_dir, "join_control", &descriptions, |x| x.join_control)?;
    generate_bool_table(code_dir, "dash", &descriptions, |x| x.dash)?;
    generate_bool_table(code_dir, "hyphen", &descriptions, |x| x.hyphen)?;
    generate_bool_table(code_dir, "quotation_mark", &descriptions, |x| x.quotation_mark)?;
    generate_bool_table(code_dir, "terminal_punctuation", &descriptions, |x| x.terminal_punctuation)?;
    generate_bool_table(code_dir, "other_math", &descriptions, |x| x.other_math)?;
    generate_bool_table(code_dir, "hex_digit", &descriptions, |x| x.hex_digit)?;
    generate_bool_table(code_dir, "ascii_hex_digit", &descriptions, |x| x.ascii_hex_digit)?;
    generate_bool_table(code_dir, "other_alphabetic", &descriptions, |x| x.other_alphabetic)?;
    generate_bool_table(code_dir, "ideographic", &descriptions, |x| x.ideographic)?;
    generate_bool_table(code_dir, "diacritic", &descriptions, |x| x.diacritic)?;
    generate_bool_table(code_dir, "extender", &descriptions, |x| x.extender)?;
    generate_bool_table(code_dir, "other_lowercase", &descriptions, |x| x.other_lowercase)?;
    generate_bool_table(code_dir, "other_uppercase", &descriptions, |x| x.other_uppercase)?;
    generate_bool_table(code_dir, "noncharacter_code_point", &descriptions, |x| x.noncharacter_code_point)?;
    generate_bool_table(code_dir, "other_grapheme_extend", &descriptions, |x| x.other_grapheme_extend)?;
    generate_bool_table(code_dir, "ids_unary_operator", &descriptions, |x| x.ids_unary_operator)?;
    generate_bool_table(code_dir, "ids_binary_operator", &descriptions, |x| x.ids_binary_operator)?;
    generate_bool_table(code_dir, "ids_trinary_operator", &descriptions, |x| x.ids_trinary_operator)?;
    generate_bool_table(code_dir, "radical", &descriptions, |x| x.radical)?;
    generate_bool_table(code_dir, "unified_ideograph", &descriptions, |x| x.unified_ideograph)?;
    generate_bool_table(code_dir, "other_default_ignorable_code_point", &descriptions, |x| { x.other_default_ignorable_code_point })?;
    generate_bool_table(code_dir, "deprecated", &descriptions, |x| x.deprecated)?;
    generate_bool_table(code_dir, "soft_dotted", &descriptions, |x| x.soft_dotted)?;
    generate_bool_table(code_dir, "logical_order_exception", &descriptions, |x| x.logical_order_exception)?;
    generate_bool_table(code_dir, "other_id_start", &descriptions, |x| x.other_id_start)?;
    generate_bool_table(code_dir, "other_id_continue", &descriptions, |x| x.other_id_continue)?;
    generate_bool_table(code_dir, "id_compat_math_continue", &descriptions, |x| x.id_compat_math_continue)?;
    generate_bool_table(code_dir, "id_compat_math_start", &descriptions, |x| x.id_compat_math_start)?;
    generate_bool_table(code_dir, "sentence_terminal", &descriptions, |x| x.sentence_terminal)?;
    generate_bool_table(code_dir, "variation_selector", &descriptions, |x| x.variation_selector)?;
    generate_bool_table(code_dir, "pattern_white_space", &descriptions, |x| x.pattern_white_space)?;
    generate_bool_table(code_dir, "pattern_syntax", &descriptions, |x| x.pattern_syntax)?;
    generate_bool_table(code_dir, "prepended_concatenation_mark", &descriptions, |x| x.prepended_concatenation_mark)?;
    generate_bool_table(code_dir, "regional_indicator", &descriptions, |x| x.regional_indicator)?;
    generate_bool_table(code_dir, "modifier_combining_mark", &descriptions, |x| x.modifier_combining_mark)?;

    generate_char_table(code_dir, "lower_case_mapping", &descriptions, |x| &x.lower_case_mapping)?;
    generate_char_table(code_dir, "upper_case_mapping", &descriptions, |x| &x.upper_case_mapping)?;
    generate_char_table(code_dir, "title_case_mapping", &descriptions, |x| &x.title_case_mapping)?;

    return Ok(());
}
