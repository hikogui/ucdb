// This file was generated by the cargo-build script.

const OTHER_LOWERCASE_CHUNK_SIZE : usize = 256;
const OTHER_LOWERCASE_COLUMN_BITS : usize = 1;
const OTHER_LOWERCASE_INDEX_LEN : usize = 482;
const OTHER_LOWERCASE_INDEX_BITS : usize = 4;

const OTHER_LOWERCASE_INDEX_BYTE_OFFSET : usize = 480;

const OTHER_LOWERCASE_DATA: [u8; 722] = [
    // Column table
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  4,  0,  4,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,255,  1,  3,  0,  0,  0, 31,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0, 32,  0,  0,  0,  0,  0,  0,  4,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 16,
      0,  0,  0,  0,  0,240,255,255,255,255,255,255,255,  7,  0,  1,  0,  0,  0,248,255,255,255,255,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  2,128,  0,  0,255, 31,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,255,255,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,255,255,255,  3,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 48,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 48,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 28,  3,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,240,  0,  2,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,185,255,255,255,255,255,253,  7,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,255,255,255,255,255,255,255, 63,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    // Index table
     16, 50, 17, 17, 17, 17, 17, 17, 20, 17, 17, 17, 17, 17, 81, 17,118, 17, 24, 17, 17, 17, 25, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,186, 17,193, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17,209, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 30,
    // Padding to handle unaligned word reads.
      0,
];

/// Get the OtherLowercase attribute for a Unicode code-point.
///
/// # Arguments
///  - `code_point` A code-point in the form of a rust `char`.
///
/// # Returns
/// bool value
#[must_use] pub const fn get_other_lowercase(code_point: char) -> bool
{
    const INDEX_MASK : usize = (1 << OTHER_LOWERCASE_INDEX_BITS) - 1;
    const COLUMN_MASK : usize = 1;

    let code_point_value = code_point as usize;
    let code_point_lo = code_point_value % OTHER_LOWERCASE_CHUNK_SIZE;
    let mut code_point_hi = code_point_value / OTHER_LOWERCASE_CHUNK_SIZE;
    if code_point_hi > OTHER_LOWERCASE_INDEX_LEN - 1 {
        code_point_hi = OTHER_LOWERCASE_INDEX_LEN - 1;
    }

    let index_offset = code_point_hi * OTHER_LOWERCASE_INDEX_BITS;
    let index_byte_offset = index_offset / 8;
    let index_bit_offset = index_offset % 8;
    let mut index: usize = 0;
    index |= (OTHER_LOWERCASE_DATA[OTHER_LOWERCASE_INDEX_BYTE_OFFSET + index_byte_offset + 1] as usize) << 8;
    index |= (OTHER_LOWERCASE_DATA[OTHER_LOWERCASE_INDEX_BYTE_OFFSET + index_byte_offset + 0] as usize) << 0;
    index >>= index_bit_offset;
    index &= INDEX_MASK;

    let column_offset = (index * OTHER_LOWERCASE_CHUNK_SIZE + code_point_lo) * OTHER_LOWERCASE_COLUMN_BITS;
    let column_byte_offset = column_offset / 8;
    let column_bit_offset = column_offset % 8;

    let mut value: usize = 0;
    value |= (OTHER_LOWERCASE_DATA[column_byte_offset + 0] as usize) << 0;
    value >>= column_bit_offset;
    value &= COLUMN_MASK;

    return match value {
        0 => false,
        _ => true,
    };
}

#[cfg(all(test, not(debug_assertions)))]
#[test]
fn other_lowercase_full_coverage()
{
    for c in '\u{0000}'..='\u{d7ff}' {
        let _ = get_other_lowercase(c);
    }
    for c in '\u{e000}'..='\u{10ffff}' {
        let _ = get_other_lowercase(c);
    }
}

