// This file was generated by the cargo-build script.

const HYPHEN_CHUNK_SIZE : usize = 128;
const HYPHEN_COLUMN_BITS : usize = 1;
const HYPHEN_INDEX_LEN : usize = 512;
const HYPHEN_INDEX_BITS : usize = 4;

const HYPHEN_INDEX_BYTE_OFFSET : usize = 144;

const HYPHEN_DATA: [u8; 401] = [
    // Column table
      0,  0,  0,  0,  0, 32,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  4,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 64,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  3,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,128,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  8,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  8,  0,  0,  0,
      0, 32,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 32,  0,  0,  0,
    // Index table
      0, 17, 17, 17, 17, 33, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 19, 17, 17, 17, 17, 17, 17, 17,
     20, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 21, 17, 97, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 23, 24,
    // Padding to handle unaligned word reads.
      0,
];

/// Get the Hyphen attribute for a Unicode code-point.
///
/// # Arguments
///  - `code_point` A code-point in the form of a rust `char`.
///
/// # Returns
/// bool value
#[must_use] pub const fn get_hyphen(code_point: char) -> bool
{
    const INDEX_MASK : usize = (1 << HYPHEN_INDEX_BITS) - 1;
    const COLUMN_MASK : usize = 1;

    let code_point_value = code_point as usize;
    let code_point_lo = code_point_value % HYPHEN_CHUNK_SIZE;
    let mut code_point_hi = code_point_value / HYPHEN_CHUNK_SIZE;
    if code_point_hi > HYPHEN_INDEX_LEN - 1 {
        code_point_hi = HYPHEN_INDEX_LEN - 1;
    }

    let index_offset = code_point_hi * HYPHEN_INDEX_BITS;
    let index_byte_offset = index_offset / 8;
    let index_bit_offset = index_offset % 8;
    let mut index: usize = 0;
    index |= (HYPHEN_DATA[HYPHEN_INDEX_BYTE_OFFSET + index_byte_offset + 1] as usize) << 8;
    index |= (HYPHEN_DATA[HYPHEN_INDEX_BYTE_OFFSET + index_byte_offset + 0] as usize) << 0;
    index >>= index_bit_offset;
    index &= INDEX_MASK;

    let column_offset = (index * HYPHEN_CHUNK_SIZE + code_point_lo) * HYPHEN_COLUMN_BITS;
    let column_byte_offset = column_offset / 8;
    let column_bit_offset = column_offset % 8;

    let mut value: usize = 0;
    value |= (HYPHEN_DATA[column_byte_offset + 0] as usize) << 0;
    value >>= column_bit_offset;
    value &= COLUMN_MASK;

    return match value {
        0 => false,
        _ => true,
    };
}

#[cfg(all(test, not(debug_assertions)))]
#[test]
fn hyphen_full_coverage()
{
    for c in '\u{0000}'..='\u{d7ff}' {
        let _ = get_hyphen(c);
    }
    for c in '\u{e000}'..='\u{10ffff}' {
        let _ = get_hyphen(c);
    }
}

