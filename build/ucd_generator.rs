
mod code_point_description;
mod parsers;
mod download;
mod column;
mod generators;

use thiserror::Error;
use code_point_description::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to write")]
    IO(#[from] std::io::Error),
    #[error("Failed to format")]
    Generator(#[from] generators::Error),
    #[error("Failed to parse")]
    Parser(#[from] parsers::Error),
}


pub fn ucd_generator(ucd_base_url: &str, ucd_version: &str, data_dir: &std::path::Path, code_dir: &std::path::Path) -> Result<(), Error>
{
    let mut code_point_descriptions = Vec::<CodePointDescription>::with_capacity(0x110000);
    code_point_descriptions.resize(0x110000, CodePointDescription::new());

    parsers::parse_east_asian_width(&ucd_base_url, &ucd_version, &data_dir, &mut code_point_descriptions)?;

    const EAST_ASIAN_WIDTH_CHUNK_SIZE : usize = 256;
    let mut east_asian_width_enum = vec!["N".to_string()];
    let mut east_asian_width_column = column::map_str_to_int(&mut east_asian_width_enum, |x| &code_point_descriptions[x].east_asian_width);
    let east_asian_width_index = column::dedup(&mut east_asian_width_column, EAST_ASIAN_WIDTH_CHUNK_SIZE);
    generators::generate_enum_table(&code_dir, "east_asian_width", &east_asian_width_enum, &east_asian_width_column, &east_asian_width_index, EAST_ASIAN_WIDTH_CHUNK_SIZE)?;

    //if let Err(e) = parse_line_break_properties(&ucd_base_url, &ucd_version, &data_dir, &mut code_point_descriptions) {
    //    return Err(e);
    //}


    return Ok(());
}

