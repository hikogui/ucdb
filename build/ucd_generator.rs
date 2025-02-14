
mod code_point_description;
use code_point_description::*;
mod parse_east_asian_width;
use parse_east_asian_width::*;
mod download;
mod column;

pub fn ucd_generator(ucd_base_url: &str, ucd_version: &str, data_dir: &std::path::Path) -> Result<(), String> {

    let mut code_point_descriptions = Vec::<CodePointDescription>::with_capacity(0x110000);
    code_point_descriptions.resize(0x110000, CodePointDescription::new());

    if let Err(e) = parse_east_asian_width(&ucd_base_url, &ucd_version, &data_dir, &mut code_point_descriptions) {
        return Err(e);
    }

    //if let Err(e) = parse_line_break_properties(&ucd_base_url, &ucd_version, &data_dir, &mut code_point_descriptions) {
    //    return Err(e);
    //}



    return Ok(());
}

