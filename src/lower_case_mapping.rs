// This file was generated by the cargo-build script.

const LOWER_CASE_MAPPING_CHUNK_SIZE : usize = 64;
const LOWER_CASE_MAPPING_COLUMN_BITS : usize = 17;
const LOWER_CASE_MAPPING_INDEX_LEN : usize = 1959;
const LOWER_CASE_MAPPING_INDEX_BITS : usize = 6;

const LOWER_CASE_MAPPING_INDEX_BYTE_OFFSET : usize = 496;

const LOWER_CASE_MAPPING_DATA: [u8; 1967] = [
    // Column table
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,250,255,255,255,  1,  0,  0,  0,  0,  0,128,115,  0,  0,  0,  0,250,255,255,255,
    191,251,191,251,255,255,191,255,127,215,191,251,255,255,255,255,255,191,221,254,255,191,255,255,255,255,255,255,255,255,255,255,
    255,215,247,255,223,215,247,181,127,255,255,255,255,255,255,255,254,255,255, 70,254, 61,  0,  0, 32,115,  0,  0,  0,  0,240,255,
    127,  0,  0,  0,  0, 96,255,255,255,255,255,255,255,255,255,255, 31,  0,  0,  0,  0,  0,250,255,255,255,255,255,255,255,255,255,
    191,219,247,255,255,255,255,255,255,255,255,255,255,255,255,255,255,251,191,251,255,255,191,  2,  0,  0,  0,  0,250,255,255,255,
    255,255,  1,  0,  0,  0,  0,  0,  0,  0,250,255,255,255,255,255,255, 15,  0,  0,  0,  0,  0,250,255,255,247,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,250,206,133,241, 41,  0,  0,  0,  0,144,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,
    255,255,255,255,243,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,251,255,255,255,255,255,251,255,255,255,255,255,
    255,255,255,255,255,255,255,255,230,254,250, 63,250,255,227,255,  0,128, 76,  8,  0,  0,250,255,255, 31,  2,  0,  0,  0,  0,  0,
      0,  0,254,255,255,255,127,  0,  0,  0,  0,  0,  0,  0,250,255,255,255,255,255,255,255,255,127,155,255,255,255,255,255,255,255,
    255,255,255,255,255,255,255,111,251,255,255,255,255,255, 27,  2,144,255,255,255,255,255,255, 21,144,255,255,255,127,  5,  0,  0,
      0,  0,  0,  0,144,255,255,255,255,255,255,255,255,255,255,247,255,255,255,255,255,255,223,255,255,255,255,255,127,  5, 64,253,
     41,  0,152, 61,  5,  0,250,255,255,255,255,255,255,255,255,255,255,255,255,255,255,  3,  0,  0,  0,  0,  0,  0,  0,250,255,255,
    255,255,255,255,  0,  0,  0,  0,  0,  0,  0,250,255,255,255,255,255, 15,  0,253,255,255,255,255,255,255,255,255,255,255,255,255,
      7,  0,  0,  0,  0,  0,250,255,255,255, 63,  0,  0,  0,  0,  0,250,255,255,255,255,255,  0,  0,  0,  0,  0,  0,250,255,255,255,
    255,255,  0,  0,232,255,255,255,255,255, 15,  0,  0,  0,  0,  0,
    // Index table
     64, 32, 12, 68, 97, 28, 72,162,  0,192,194, 52,206,  3, 69,210, 68,  1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 84,  0,  0,  0,  0,  0,  0,  0,  0, 88,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,112,  1,  0,150,  1,218,198,117,222,  7,134,
      0,  0,  0,128, 56,  2,  0,  0,  0,  0,  0,  0,  0,  0,144,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,165,121,162, 41,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,128,186,  2,108,235,190,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0, 28,  3,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,128, 12,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 51, 13,212,  0, 96,  3,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,220,  0,158,  3,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,232,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,192, 14,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,124, 15,  0,
    // Padding to handle unaligned word reads.
      0,
];

/// Get the LowerCaseMapping attribute for a Unicode code-point.
///
/// # Arguments
///  - `code_point` A code-point in the form of a rust `char`.
///
/// # Returns
/// bool value
#[must_use] pub const fn get_lower_case_mapping(code_point: char) -> Optional<char>
{
    const INDEX_MASK : usize = (1 << LOWER_CASE_MAPPING_INDEX_BITS) - 1;
    const COLUMN_MASK : usize = (1 << LOWER_CASE_MAPPING_COLUMN_BITS) - 1;

    let code_point_value = code_point as usize;
    let code_point_lo = code_point_value % LOWER_CASE_MAPPING_CHUNK_SIZE;
    let mut code_point_hi = code_point_value / LOWER_CASE_MAPPING_CHUNK_SIZE;
    if code_point_hi > LOWER_CASE_MAPPING_INDEX_LEN - 1 {
        code_point_hi = LOWER_CASE_MAPPING_INDEX_LEN - 1;
    }

    let index_offset = code_point_hi * LOWER_CASE_MAPPING_INDEX_BITS;
    let index_byte_offset = index_offset / 8;
    let index_bit_offset = index_offset % 8;
    let mut index: usize = 0;
    index |= (LOWER_CASE_MAPPING_DATA[LOWER_CASE_MAPPING_INDEX_BYTE_OFFSET + index_byte_offset + 1] as usize) << 8;
    index |= (LOWER_CASE_MAPPING_DATA[LOWER_CASE_MAPPING_INDEX_BYTE_OFFSET + index_byte_offset + 0] as usize) << 0;
    index >>= index_bit_offset;
    index &= INDEX_MASK;

    let column_offset = (index * LOWER_CASE_MAPPING_CHUNK_SIZE + code_point_lo) * LOWER_CASE_MAPPING_COLUMN_BITS;
    let column_byte_offset = column_offset / 8;
    let column_bit_offset = column_offset % 8;

    let mut value: usize = 0;
    value |= (LOWER_CASE_MAPPING_DATA[column_byte_offset + 3] as usize) << 24;
    value |= (LOWER_CASE_MAPPING_DATA[column_byte_offset + 2] as usize) << 16;
    value |= (LOWER_CASE_MAPPING_DATA[column_byte_offset + 1] as usize) << 8;
    value |= (LOWER_CASE_MAPPING_DATA[column_byte_offset + 0] as usize) << 0;
    value >>= column_bit_offset;
    value &= COLUMN_MASK;

    return match value {
        0 => None,
        _ => char::from_u32(value as u32),
    };
}

#[cfg(all(test, not(debug_assertions)))]
#[test]
fn lower_case_mapping_full_coverage()
{
    for c in '\u{0000}'..='\u{d7ff}' {
        let _ = get_lower_case_mapping(c);
    }
    for c in '\u{e000}'..='\u{10ffff}' {
        let _ = get_lower_case_mapping(c);
    }
}

