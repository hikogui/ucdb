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

    generators::generate_enum_table(
        &code_dir,
        name,
        &enum_table,
        &dedup,
        dedup_bits,
        &index,
        index_bits,
        chunk_size,
    )?;

    return Ok(());
}

pub fn ucd_generator(
    ucd_base_url: &str,
    ucd_version: &str,
    data_dir: &std::path::Path,
    code_dir: &std::path::Path,
) -> Result<(), Error> {
    let mut code_point_descriptions = Vec::<CodePointDescription>::with_capacity(0x110000);
    code_point_descriptions.resize(0x110000, CodePointDescription::new());

    parsers::parse_east_asian_width(
        &ucd_base_url,
        &ucd_version,
        &data_dir,
        &mut code_point_descriptions,
    )?;

    generate_enum_table(code_dir, "east_asian_width", vec!["N".to_string()], |x| {
        &code_point_descriptions[x].east_asian_width
    })?;

    //if let Err(e) = parse_line_break_properties(&ucd_base_url, &ucd_version, &data_dir, &mut code_point_descriptions) {
    //    return Err(e);
    //}

    return Ok(());
}
