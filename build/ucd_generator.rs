
mod code_point_description;
use code_point_description::*;
mod parse_east_asian_width;
use parse_east_asian_width::*;

pub fn ucd_generator(ucd_version: &str) -> Result<(), String> {

    let mut code_point_descriptions = Vec::<CodePointDescription>::with_capacity(0x11000);

    if let Err(e) = parse_east_asian_width(&ucd_version, &mut code_point_descriptions) {
        return Err(e);
    }

    return Ok(());
}

