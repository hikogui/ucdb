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
    op: impl Fn(usize) -> &'a String,
) -> Result<(), Error> {
    let column = column::map_str_to_int(&mut enum_table, op);

    let (dedup, dedup_bits, index, index_bits, chunk_size) = column::dedup_best_fit(&column);

    generators::generate_enum_table(&code_dir, name, &enum_table, &dedup, dedup_bits, &index, index_bits, chunk_size)?;

    return Ok(());
}

fn generate_bool_table(code_dir: &std::path::Path, name: &str, op: impl Fn(usize) -> bool) -> Result<(), Error> {
    let column = column::map_bool_to_int(op);

    let (dedup, dedup_bits, index, index_bits, chunk_size) = column::dedup_best_fit(&column);
    assert!(dedup_bits == 1);

    generators::generate_bool_table(&code_dir, name, &dedup, &index, index_bits, chunk_size)?;

    return Ok(());
}

pub fn build(ucd_base_url: &str, ucd_version: &str, data_dir: &std::path::Path, code_dir: &std::path::Path) -> Result<(), Error> {
    let mut code_point_descriptions = Vec::<CodePointDescription>::with_capacity(0x110000);
    code_point_descriptions.resize(0x110000, CodePointDescription::new());

    parsers::parse_single_column(
        &format!("{}/{}/ucd/EastAsianWidth.txt", &ucd_base_url, &ucd_version),
        &data_dir.join(&ucd_version).join("ucd").join("EastAsianWidth.txt"),
        &mut code_point_descriptions,
        |x| &mut x.east_asian_width,
    )?;

    parsers::parse_single_column(
        &format!("{}/{}/ucd/LineBreak.txt", &ucd_base_url, &ucd_version),
        &data_dir.join(&ucd_version).join("ucd").join("LineBreak.txt"),
        &mut code_point_descriptions,
        |x| &mut x.line_break,
    )?;

    parsers::parse_single_column(
        &format!("{}/{}/ucd/auxiliary/WordBreakProperty.txt", &ucd_base_url, &ucd_version),
        &data_dir
            .join(&ucd_version)
            .join("ucd")
            .join("auxiliary")
            .join("WordBreakProperty.txt"),
        &mut code_point_descriptions,
        |x| &mut x.word_break,
    )?;

    parsers::parse_single_column(
        &format!("{}/{}/ucd/auxiliary/SentenceBreakProperty.txt", &ucd_base_url, &ucd_version),
        &data_dir
            .join(&ucd_version)
            .join("ucd")
            .join("auxiliary")
            .join("SentenceBreakProperty.txt"),
        &mut code_point_descriptions,
        |x| &mut x.sentence_break,
    )?;

    parsers::parse_single_column(
        &format!("{}/{}/ucd/auxiliary/GraphemeBreakProperty.txt", &ucd_base_url, &ucd_version),
        &data_dir
            .join(&ucd_version)
            .join("ucd")
            .join("auxiliary")
            .join("GraphemeBreakProperty.txt"),
        &mut code_point_descriptions,
        |x| &mut x.grapheme_break,
    )?;

    parsers::parse_single_column(
        &format!("{}/{}/ucd/Scripts.txt", &ucd_base_url, &ucd_version),
        &data_dir.join(&ucd_version).join("ucd").join("Scripts.txt"),
        &mut code_point_descriptions,
        |x| &mut x.script,
    )?;

    parsers::parse_existance_column(
        &format!("{}/{}/ucd/CompositionExclusions.txt", &ucd_base_url, &ucd_version),
        &data_dir.join(&ucd_version).join("ucd").join("CompositionExclusions.txt"),
        &mut code_point_descriptions,
        |x| &mut x.composition_exclusion,
    )?;

    parsers::parse_prop_list_columns(
        &format!("{}/{}/ucd/PropList.txt", &ucd_base_url, &ucd_version),
        &data_dir.join(&ucd_version).join("ucd").join("PropList.txt"),
        &mut code_point_descriptions,
    )?;

    generate_enum_table(code_dir, "east_asian_width", vec!["N".to_string()], |x| &code_point_descriptions[x].east_asian_width)?;
    generate_enum_table(code_dir, "line_break", vec!["N".to_string()], |x| &code_point_descriptions[x].line_break)?;
    generate_enum_table(code_dir, "word_break", vec!["N".to_string()], |x| &code_point_descriptions[x].word_break)?;
    generate_enum_table(code_dir, "sentence_break", vec!["N".to_string()], |x| &code_point_descriptions[x].sentence_break)?;
    generate_enum_table(code_dir, "grapheme_break", vec!["N".to_string()], |x| &code_point_descriptions[x].grapheme_break)?;
    generate_enum_table(code_dir, "script", vec!["N".to_string()], |x| &code_point_descriptions[x].script)?;

    generate_bool_table(code_dir, "composition_exclusion", |x| code_point_descriptions[x].composition_exclusion)?;
    generate_bool_table(code_dir, "white_space", |x| code_point_descriptions[x].white_space)?;
    generate_bool_table(code_dir, "bidi_control", |x| code_point_descriptions[x].bidi_control)?;
    generate_bool_table(code_dir, "join_control", |x| code_point_descriptions[x].join_control)?;
    generate_bool_table(code_dir, "dash", |x| code_point_descriptions[x].dash)?;
    generate_bool_table(code_dir, "hyphen", |x| code_point_descriptions[x].hyphen)?;
    generate_bool_table(code_dir, "quotation_mark", |x| code_point_descriptions[x].quotation_mark)?;
    generate_bool_table(code_dir, "terminal_punctuation", |x| code_point_descriptions[x].terminal_punctuation)?;
    generate_bool_table(code_dir, "other_math", |x| code_point_descriptions[x].other_math)?;
    generate_bool_table(code_dir, "hex_digit", |x| code_point_descriptions[x].hex_digit)?;
    generate_bool_table(code_dir, "ascii_hex_digit", |x| code_point_descriptions[x].ascii_hex_digit)?;
    generate_bool_table(code_dir, "other_alphabetic", |x| code_point_descriptions[x].other_alphabetic)?;
    generate_bool_table(code_dir, "ideographic", |x| code_point_descriptions[x].ideographic)?;
    generate_bool_table(code_dir, "diacritic", |x| code_point_descriptions[x].diacritic)?;
    generate_bool_table(code_dir, "extender", |x| code_point_descriptions[x].extender)?;
    generate_bool_table(code_dir, "other_lowercase", |x| code_point_descriptions[x].other_lowercase)?;
    generate_bool_table(code_dir, "other_uppercase", |x| code_point_descriptions[x].other_uppercase)?;
    generate_bool_table(code_dir, "noncharacter_code_point", |x| code_point_descriptions[x].noncharacter_code_point)?;
    generate_bool_table(code_dir, "other_grapheme_extend", |x| code_point_descriptions[x].other_grapheme_extend)?;
    generate_bool_table(code_dir, "ids_unary_operator", |x| code_point_descriptions[x].ids_unary_operator)?;
    generate_bool_table(code_dir, "ids_binary_operator", |x| code_point_descriptions[x].ids_binary_operator)?;
    generate_bool_table(code_dir, "ids_trinary_operator", |x| code_point_descriptions[x].ids_trinary_operator)?;
    generate_bool_table(code_dir, "radical", |x| code_point_descriptions[x].radical)?;
    generate_bool_table(code_dir, "unified_ideograph", |x| code_point_descriptions[x].unified_ideograph)?;
    generate_bool_table(code_dir, "other_default_ignorable_code_point", |x| {
        code_point_descriptions[x].other_default_ignorable_code_point
    })?;
    generate_bool_table(code_dir, "deprecated", |x| code_point_descriptions[x].deprecated)?;
    generate_bool_table(code_dir, "soft_dotted", |x| code_point_descriptions[x].soft_dotted)?;
    generate_bool_table(code_dir, "logical_order_exception", |x| code_point_descriptions[x].logical_order_exception)?;
    generate_bool_table(code_dir, "other_id_start", |x| code_point_descriptions[x].other_id_start)?;
    generate_bool_table(code_dir, "other_id_continue", |x| code_point_descriptions[x].other_id_continue)?;
    generate_bool_table(code_dir, "id_compat_math_continue", |x| code_point_descriptions[x].id_compat_math_continue)?;
    generate_bool_table(code_dir, "id_compat_math_start", |x| code_point_descriptions[x].id_compat_math_start)?;
    generate_bool_table(code_dir, "sentence_terminal", |x| code_point_descriptions[x].sentence_terminal)?;
    generate_bool_table(code_dir, "variation_selector", |x| code_point_descriptions[x].variation_selector)?;
    generate_bool_table(code_dir, "pattern_white_space", |x| code_point_descriptions[x].pattern_white_space)?;
    generate_bool_table(code_dir, "pattern_syntax", |x| code_point_descriptions[x].pattern_syntax)?;
    generate_bool_table(code_dir, "prepended_concatenation_mark", |x| code_point_descriptions[x].prepended_concatenation_mark)?;
    generate_bool_table(code_dir, "regional_indicator", |x| code_point_descriptions[x].regional_indicator)?;
    generate_bool_table(code_dir, "modifier_combining_mark", |x| code_point_descriptions[x].modifier_combining_mark)?;

    return Ok(());
}
